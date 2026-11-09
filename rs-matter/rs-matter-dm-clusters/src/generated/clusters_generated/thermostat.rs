# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ACErrorCodeBitmap : u32 {
        const COMPRESSOR_FAIL = 1 ;
        const ROOM_SENSOR_FAIL = 2 ;
        const OUTDOOR_SENSOR_FAIL = 4 ;
        const COIL_SENSOR_FAIL = 8 ;
        const FAN_FAIL = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ACErrorCodeBitmap : u32 {
        const COMPRESSOR_FAIL = 1 ;
        const ROOM_SENSOR_FAIL = 2 ;
        const OUTDOOR_SENSOR_FAIL = 4 ;
        const COIL_SENSOR_FAIL = 8 ;
        const FAN_FAIL = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ACErrorCodeBitmap , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const HEATING = 1 ;
        const COOLING = 2 ;
        const OCCUPANCY = 4 ;
        const SCHEDULE_CONFIGURATION = 8 ;
        const SETBACK = 16 ;
        const AUTO_MODE = 32 ;
        const LOCAL_TEMPERATURE_NOT_EXPOSED = 64 ;
        const MATTER_SCHEDULE_CONFIGURATION = 128 ;
        const PRESETS = 256 ;
        const EVENTS = 512 ;
        const THERMOSTAT_SUGGESTIONS = 1024 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const HEATING = 1 ;
        const COOLING = 2 ;
        const OCCUPANCY = 4 ;
        const SCHEDULE_CONFIGURATION = 8 ;
        const SETBACK = 16 ;
        const AUTO_MODE = 32 ;
        const LOCAL_TEMPERATURE_NOT_EXPOSED = 64 ;
        const MATTER_SCHEDULE_CONFIGURATION = 128 ;
        const PRESETS = 256 ;
        const EVENTS = 512 ;
        const THERMOSTAT_SUGGESTIONS = 1024 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct HVACSystemTypeBitmap : u8 {
        const COOLING_STAGE = 3 ;
        const HEATING_STAGE = 12 ;
        const HEATING_IS_HEAT_PUMP = 16 ;
        const HEATING_USES_FUEL = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct HVACSystemTypeBitmap : u8 {
        const COOLING_STAGE = 3 ;
        const HEATING_STAGE = 12 ;
        const HEATING_IS_HEAT_PUMP = 16 ;
        const HEATING_USES_FUEL = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (HVACSystemTypeBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct OccupancyBitmap : u8 {
        const OCCUPIED = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct OccupancyBitmap : u8 {
        const OCCUPIED = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (OccupancyBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct PresetTypeFeaturesBitmap : u16 {
        const AUTOMATIC = 1 ;
        const SUPPORTS_NAMES = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct PresetTypeFeaturesBitmap : u16 {
        const AUTOMATIC = 1 ;
        const SUPPORTS_NAMES = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (PresetTypeFeaturesBitmap , u16 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ProgrammingOperationModeBitmap : u8 {
        const SCHEDULE_ACTIVE = 1 ;
        const AUTO_RECOVERY = 2 ;
        const ECONOMY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ProgrammingOperationModeBitmap : u8 {
        const SCHEDULE_ACTIVE = 1 ;
        const AUTO_RECOVERY = 2 ;
        const ECONOMY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ProgrammingOperationModeBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct RelayStateBitmap : u16 {
        const HEAT = 1 ;
        const COOL = 2 ;
        const FAN = 4 ;
        const HEAT_STAGE_2 = 8 ;
        const COOL_STAGE_2 = 16 ;
        const FAN_STAGE_2 = 32 ;
        const FAN_STAGE_3 = 64 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct RelayStateBitmap : u16 {
        const HEAT = 1 ;
        const COOL = 2 ;
        const FAN = 4 ;
        const HEAT_STAGE_2 = 8 ;
        const COOL_STAGE_2 = 16 ;
        const FAN_STAGE_2 = 32 ;
        const FAN_STAGE_3 = 64 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (RelayStateBitmap , u16 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct RemoteSensingBitmap : u8 {
        const LOCAL_TEMPERATURE = 1 ;
        const OUTDOOR_TEMPERATURE = 2 ;
        const OCCUPANCY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct RemoteSensingBitmap : u8 {
        const LOCAL_TEMPERATURE = 1 ;
        const OUTDOOR_TEMPERATURE = 2 ;
        const OCCUPANCY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (RemoteSensingBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ScheduleDayOfWeekBitmap : u8 {
        const SUNDAY = 1 ;
        const MONDAY = 2 ;
        const TUESDAY = 4 ;
        const WEDNESDAY = 8 ;
        const THURSDAY = 16 ;
        const FRIDAY = 32 ;
        const SATURDAY = 64 ;
        const AWAY = 128 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ScheduleDayOfWeekBitmap : u8 {
        const SUNDAY = 1 ;
        const MONDAY = 2 ;
        const TUESDAY = 4 ;
        const WEDNESDAY = 8 ;
        const THURSDAY = 16 ;
        const FRIDAY = 32 ;
        const SATURDAY = 64 ;
        const AWAY = 128 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ScheduleDayOfWeekBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ScheduleModeBitmap : u8 {
        const HEAT_SETPOINT_PRESENT = 1 ;
        const COOL_SETPOINT_PRESENT = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ScheduleModeBitmap : u8 {
        const HEAT_SETPOINT_PRESENT = 1 ;
        const COOL_SETPOINT_PRESENT = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ScheduleModeBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ScheduleTypeFeaturesBitmap : u16 {
        const SUPPORTS_PRESETS = 1 ;
        const SUPPORTS_SETPOINTS = 2 ;
        const SUPPORTS_NAMES = 4 ;
        const SUPPORTS_OFF = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ScheduleTypeFeaturesBitmap : u16 {
        const SUPPORTS_PRESETS = 1 ;
        const SUPPORTS_SETPOINTS = 2 ;
        const SUPPORTS_NAMES = 4 ;
        const SUPPORTS_OFF = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ScheduleTypeFeaturesBitmap , u16 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ThermostatSuggestionNotFollowingReasonBitmap : u16 {
        const DEMAND_RESPONSE_EVENT = 1 ;
        const ONGOING_HOLD = 2 ;
        const SCHEDULE = 4 ;
        const OCCUPANCY = 8 ;
        const VACATION_MODE = 16 ;
        const TIME_OF_USE_COST_SAVINGS = 32 ;
        const PRE_COOLING_OR_PRE_HEATING = 64 ;
        const CONFLICTING_SUGGESTIONS = 128 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ThermostatSuggestionNotFollowingReasonBitmap : u16 {
        const DEMAND_RESPONSE_EVENT = 1 ;
        const ONGOING_HOLD = 2 ;
        const SCHEDULE = 4 ;
        const OCCUPANCY = 8 ;
        const VACATION_MODE = 16 ;
        const TIME_OF_USE_COST_SAVINGS = 32 ;
        const PRE_COOLING_OR_PRE_HEATING = 64 ;
        const CONFLICTING_SUGGESTIONS = 128 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ThermostatSuggestionNotFollowingReasonBitmap , u16 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ACCapacityFormatEnum {
    # [enumval (0 ) ] BTUh = 0 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ACCompressorTypeEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] T1 = 1 , # [enumval (2 ) ] T2 = 2 , # [enumval (3 ) ] T3 = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ACLouverPositionEnum {
    # [enumval (1 ) ] Closed = 1 , # [enumval (2 ) ] Open = 2 , # [enumval (3 ) ] Quarter = 3 , # [enumval (4 ) ] Half = 4 , # [enumval (5 ) ] ThreeQuarters = 5 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ACRefrigerantTypeEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] R22 = 1 , # [enumval (2 ) ] R410a = 2 , # [enumval (3 ) ] R407c = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ACTypeEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] CoolingFixed = 1 , # [enumval (2 ) ] HeatPumpFixed = 2 , # [enumval (3 ) ] CoolingInverter = 3 , # [enumval (4 ) ] HeatPumpInverter = 4 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ControlSequenceOfOperationEnum {
    # [enumval (0 ) ] CoolingOnly = 0 , # [enumval (1 ) ] CoolingWithReheat = 1 , # [enumval (2 ) ] HeatingOnly = 2 , # [enumval (3 ) ] HeatingWithReheat = 3 , # [enumval (4 ) ] CoolingAndHeating = 4 , # [enumval (5 ) ] CoolingAndHeatingWithReheat = 5 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PresetScenarioEnum {
    # [enumval (1 ) ] Occupied = 1 , # [enumval (2 ) ] Unoccupied = 2 , # [enumval (3 ) ] Sleep = 3 , # [enumval (4 ) ] Wake = 4 , # [enumval (5 ) ] Vacation = 5 , # [enumval (6 ) ] GoingToSleep = 6 , # [enumval (254 ) ] UserDefined = 254 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetpointChangeSourceEnum {
    # [enumval (0 ) ] Manual = 0 , # [enumval (1 ) ] Schedule = 1 , # [enumval (2 ) ] External = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetpointRaiseLowerModeEnum {
    # [enumval (0 ) ] Heat = 0 , # [enumval (1 ) ] Cool = 1 , # [enumval (2 ) ] Both = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StartOfWeekEnum {
    # [enumval (0 ) ] Sunday = 0 , # [enumval (1 ) ] Monday = 1 , # [enumval (2 ) ] Tuesday = 2 , # [enumval (3 ) ] Wednesday = 3 , # [enumval (4 ) ] Thursday = 4 , # [enumval (5 ) ] Friday = 5 , # [enumval (6 ) ] Saturday = 6 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SystemModeEnum {
    # [enumval (0 ) ] Off = 0 , # [enumval (1 ) ] Auto = 1 , # [enumval (3 ) ] Cool = 3 , # [enumval (4 ) ] Heat = 4 , # [enumval (5 ) ] EmergencyHeat = 5 , # [enumval (6 ) ] Precooling = 6 , # [enumval (7 ) ] FanOnly = 7 , # [enumval (8 ) ] Dry = 8 , # [enumval (9 ) ] Sleep = 9 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TemperatureSetpointHoldEnum {
    # [enumval (0 ) ] SetpointHoldOff = 0 , # [enumval (1 ) ] SetpointHoldOn = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ThermostatRunningModeEnum {
    # [enumval (0 ) ] Off = 0 , # [enumval (3 ) ] Cool = 3 , # [enumval (4 ) ] Heat = 4 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ScheduleTransitionStructTag {
    DayOfWeek = 0 , TransitionTime = 1 , PresetHandle = 2 , SystemMode = 3 , CoolingSetpoint = 4 , HeatingSetpoint = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ScheduleStructTag {
    ScheduleHandle = 0 , SystemMode = 1 , Name = 2 , PresetHandle = 3 , Transitions = 4 , BuiltIn = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PresetStructTag {
    PresetHandle = 0 , PresetScenario = 1 , Name = 2 , CoolingSetpoint = 3 , HeatingSetpoint = 4 , BuiltIn = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PresetTypeStructTag {
    PresetScenario = 0 , NumberOfPresets = 1 , PresetTypeFeatures = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ScheduleTypeStructTag {
    SystemMode = 0 , NumberOfSchedules = 1 , ScheduleTypeFeatures = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ThermostatSuggestionStructTag {
    UniqueId = 0 , PresetHandle = 1 , EffectiveTime = 2 , ExpirationTime = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WeeklyScheduleTransitionStructTag {
    TransitionTime = 0 , HeatSetpoint = 1 , CoolSetpoint = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetpointRaiseLowerRequestTag {
    Mode = 0 , Amount = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GetWeeklyScheduleResponseTag {
    NumberOfTransitionsForSequence = 0 , DayOfWeekForSequence = 1 , ModeForSequence = 2 , Transitions = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetWeeklyScheduleRequestTag {
    NumberOfTransitionsForSequence = 0 , DayOfWeekForSequence = 1 , ModeForSequence = 2 , Transitions = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GetWeeklyScheduleRequestTag {
    DaysToReturn = 0 , ModeToReturn = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddThermostatSuggestionResponseTag {
    UniqueId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetActiveScheduleRequestRequestTag {
    ScheduleHandle = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetActivePresetRequestRequestTag {
    PresetHandle = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddThermostatSuggestionRequestTag {
    PresetHandle = 0 , EffectiveTime = 1 , ExpirationInMinutes = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RemoveThermostatSuggestionRequestTag {
    UniqueId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AtomicResponseTag {
    StatusCode = 0 , AttributeStatus = 1 , Timeout = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AtomicRequestRequestTag {
    RequestType = 0 , AttributeRequests = 1 , Timeout = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ScheduleTransitionStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ScheduleTransitionStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn day_of_week (& self ) -> Result < ScheduleDayOfWeekBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn preset_handle (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn system_mode (& self ) -> Result < Option < SystemModeEnum > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn cooling_setpoint (& self ) -> Result < Option < i16 > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn heating_setpoint (& self ) -> Result < Option < i16 > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ScheduleTransitionStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ScheduleTransitionStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ScheduleTransitionStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ScheduleTransitionStruct" ) ? ;
        match self . day_of_week () {
            Ok (value ) => write ! (f , "{}: {:?}," , "day_of_week" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "day_of_week" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . preset_handle () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "preset_handle" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "preset_handle" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        match self . system_mode () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "system_mode" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "system_mode" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) ? , 
        }
        match self . cooling_setpoint () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cooling_setpoint" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cooling_setpoint" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cooling_setpoint" , e . code () ) ? , 
        }
        match self . heating_setpoint () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "heating_setpoint" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "heating_setpoint" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "heating_setpoint" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ScheduleTransitionStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ScheduleTransitionStruct" ) ;
        match self . day_of_week () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "day_of_week" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "day_of_week" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . preset_handle () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "preset_handle" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "preset_handle" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        match self . system_mode () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "system_mode" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "system_mode" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) , 
        }
        match self . cooling_setpoint () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cooling_setpoint" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cooling_setpoint" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cooling_setpoint" , e . code () ) , 
        }
        match self . heating_setpoint () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "heating_setpoint" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "heating_setpoint" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "heating_setpoint" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ScheduleStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ScheduleStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn schedule_handle (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn system_mode (& self ) -> Result < SystemModeEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn name (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn preset_handle (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn transitions (& self ) -> Result < crate :: tlv :: TLVArray < '_ , ScheduleTransitionStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn built_in (& self ) -> Result < crate :: tlv :: Nullable < bool > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ScheduleStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ScheduleStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ScheduleStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ScheduleStruct" ) ? ;
        match self . schedule_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "schedule_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "schedule_handle" , e . code () ) ? , 
        }
        match self . system_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "system_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) ? , 
        }
        match self . name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . preset_handle () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "preset_handle" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "preset_handle" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        match self . transitions () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transitions" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) ? , 
        }
        match self . built_in () {
            Ok (value ) => write ! (f , "{}: {:?}," , "built_in" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "built_in" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ScheduleStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ScheduleStruct" ) ;
        match self . schedule_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "schedule_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "schedule_handle" , e . code () ) , 
        }
        match self . system_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "system_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) , 
        }
        match self . name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . preset_handle () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "preset_handle" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "preset_handle" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        match self . transitions () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transitions" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) , 
        }
        match self . built_in () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "built_in" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "built_in" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PresetStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PresetStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn preset_handle (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn preset_scenario (& self ) -> Result < PresetScenarioEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn name (& self ) -> Result < Option < crate :: tlv :: Nullable < crate :: tlv :: Utf8Str < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn cooling_setpoint (& self ) -> Result < Option < i16 > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn heating_setpoint (& self ) -> Result < Option < i16 > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn built_in (& self ) -> Result < crate :: tlv :: Nullable < bool > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PresetStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PresetStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PresetStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PresetStruct" ) ? ;
        match self . preset_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        match self . preset_scenario () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_scenario" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_scenario" , e . code () ) ? , 
        }
        match self . name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . cooling_setpoint () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cooling_setpoint" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cooling_setpoint" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cooling_setpoint" , e . code () ) ? , 
        }
        match self . heating_setpoint () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "heating_setpoint" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "heating_setpoint" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "heating_setpoint" , e . code () ) ? , 
        }
        match self . built_in () {
            Ok (value ) => write ! (f , "{}: {:?}," , "built_in" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "built_in" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PresetStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PresetStruct" ) ;
        match self . preset_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        match self . preset_scenario () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_scenario" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_scenario" , e . code () ) , 
        }
        match self . name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . cooling_setpoint () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cooling_setpoint" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cooling_setpoint" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cooling_setpoint" , e . code () ) , 
        }
        match self . heating_setpoint () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "heating_setpoint" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "heating_setpoint" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "heating_setpoint" , e . code () ) , 
        }
        match self . built_in () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "built_in" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "built_in" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PresetTypeStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PresetTypeStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn preset_scenario (& self ) -> Result < PresetScenarioEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn number_of_presets (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn preset_type_features (& self ) -> Result < PresetTypeFeaturesBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PresetTypeStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PresetTypeStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PresetTypeStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PresetTypeStruct" ) ? ;
        match self . preset_scenario () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_scenario" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_scenario" , e . code () ) ? , 
        }
        match self . number_of_presets () {
            Ok (value ) => write ! (f , "{}: {:?}," , "number_of_presets" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "number_of_presets" , e . code () ) ? , 
        }
        match self . preset_type_features () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_type_features" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_type_features" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PresetTypeStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PresetTypeStruct" ) ;
        match self . preset_scenario () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_scenario" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_scenario" , e . code () ) , 
        }
        match self . number_of_presets () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "number_of_presets" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "number_of_presets" , e . code () ) , 
        }
        match self . preset_type_features () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_type_features" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_type_features" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ScheduleTypeStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ScheduleTypeStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn system_mode (& self ) -> Result < SystemModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn number_of_schedules (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn schedule_type_features (& self ) -> Result < ScheduleTypeFeaturesBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ScheduleTypeStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ScheduleTypeStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ScheduleTypeStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ScheduleTypeStruct" ) ? ;
        match self . system_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "system_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) ? , 
        }
        match self . number_of_schedules () {
            Ok (value ) => write ! (f , "{}: {:?}," , "number_of_schedules" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "number_of_schedules" , e . code () ) ? , 
        }
        match self . schedule_type_features () {
            Ok (value ) => write ! (f , "{}: {:?}," , "schedule_type_features" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "schedule_type_features" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ScheduleTypeStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ScheduleTypeStruct" ) ;
        match self . system_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "system_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) , 
        }
        match self . number_of_schedules () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "number_of_schedules" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "number_of_schedules" , e . code () ) , 
        }
        match self . schedule_type_features () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "schedule_type_features" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "schedule_type_features" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ThermostatSuggestionStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ThermostatSuggestionStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn unique_id (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn preset_handle (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn effective_time (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn expiration_time (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ThermostatSuggestionStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ThermostatSuggestionStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ThermostatSuggestionStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ThermostatSuggestionStruct" ) ? ;
        match self . unique_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "unique_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) ? , 
        }
        match self . preset_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        match self . effective_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "effective_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "effective_time" , e . code () ) ? , 
        }
        match self . expiration_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "expiration_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "expiration_time" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ThermostatSuggestionStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ThermostatSuggestionStruct" ) ;
        match self . unique_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "unique_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) , 
        }
        match self . preset_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        match self . effective_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "effective_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "effective_time" , e . code () ) , 
        }
        match self . expiration_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "expiration_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "expiration_time" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct WeeklyScheduleTransitionStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > WeeklyScheduleTransitionStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn heat_setpoint (& self ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn cool_setpoint (& self ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for WeeklyScheduleTransitionStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for WeeklyScheduleTransitionStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for WeeklyScheduleTransitionStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "WeeklyScheduleTransitionStruct" ) ? ;
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . heat_setpoint () {
            Ok (value ) => write ! (f , "{}: {:?}," , "heat_setpoint" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "heat_setpoint" , e . code () ) ? , 
        }
        match self . cool_setpoint () {
            Ok (value ) => write ! (f , "{}: {:?}," , "cool_setpoint" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cool_setpoint" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for WeeklyScheduleTransitionStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "WeeklyScheduleTransitionStruct" ) ;
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . heat_setpoint () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "heat_setpoint" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "heat_setpoint" , e . code () ) , 
        }
        match self . cool_setpoint () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "cool_setpoint" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cool_setpoint" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetpointRaiseLowerRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetpointRaiseLowerRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn mode (& self ) -> Result < SetpointRaiseLowerModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn amount (& self ) -> Result < i8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetpointRaiseLowerRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetpointRaiseLowerRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetpointRaiseLowerRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetpointRaiseLowerRequest" ) ? ;
        match self . mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mode" , e . code () ) ? , 
        }
        match self . amount () {
            Ok (value ) => write ! (f , "{}: {:?}," , "amount" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "amount" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetpointRaiseLowerRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetpointRaiseLowerRequest" ) ;
        match self . mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mode" , e . code () ) , 
        }
        match self . amount () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "amount" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "amount" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GetWeeklyScheduleResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GetWeeklyScheduleResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn number_of_transitions_for_sequence (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn day_of_week_for_sequence (& self ) -> Result < ScheduleDayOfWeekBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn mode_for_sequence (& self ) -> Result < ScheduleModeBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn transitions (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WeeklyScheduleTransitionStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GetWeeklyScheduleResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GetWeeklyScheduleResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GetWeeklyScheduleResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GetWeeklyScheduleResponse" ) ? ;
        match self . number_of_transitions_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "number_of_transitions_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "number_of_transitions_for_sequence" , e . code () ) ? , 
        }
        match self . day_of_week_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "day_of_week_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "day_of_week_for_sequence" , e . code () ) ? , 
        }
        match self . mode_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mode_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mode_for_sequence" , e . code () ) ? , 
        }
        match self . transitions () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transitions" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GetWeeklyScheduleResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GetWeeklyScheduleResponse" ) ;
        match self . number_of_transitions_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "number_of_transitions_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "number_of_transitions_for_sequence" , e . code () ) , 
        }
        match self . day_of_week_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "day_of_week_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "day_of_week_for_sequence" , e . code () ) , 
        }
        match self . mode_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mode_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mode_for_sequence" , e . code () ) , 
        }
        match self . transitions () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transitions" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetWeeklyScheduleRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetWeeklyScheduleRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn number_of_transitions_for_sequence (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn day_of_week_for_sequence (& self ) -> Result < ScheduleDayOfWeekBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn mode_for_sequence (& self ) -> Result < ScheduleModeBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn transitions (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WeeklyScheduleTransitionStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetWeeklyScheduleRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetWeeklyScheduleRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetWeeklyScheduleRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetWeeklyScheduleRequest" ) ? ;
        match self . number_of_transitions_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "number_of_transitions_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "number_of_transitions_for_sequence" , e . code () ) ? , 
        }
        match self . day_of_week_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "day_of_week_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "day_of_week_for_sequence" , e . code () ) ? , 
        }
        match self . mode_for_sequence () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mode_for_sequence" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mode_for_sequence" , e . code () ) ? , 
        }
        match self . transitions () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transitions" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetWeeklyScheduleRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetWeeklyScheduleRequest" ) ;
        match self . number_of_transitions_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "number_of_transitions_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "number_of_transitions_for_sequence" , e . code () ) , 
        }
        match self . day_of_week_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "day_of_week_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "day_of_week_for_sequence" , e . code () ) , 
        }
        match self . mode_for_sequence () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mode_for_sequence" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mode_for_sequence" , e . code () ) , 
        }
        match self . transitions () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transitions" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transitions" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GetWeeklyScheduleRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GetWeeklyScheduleRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn days_to_return (& self ) -> Result < ScheduleDayOfWeekBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn mode_to_return (& self ) -> Result < ScheduleModeBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GetWeeklyScheduleRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GetWeeklyScheduleRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GetWeeklyScheduleRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GetWeeklyScheduleRequest" ) ? ;
        match self . days_to_return () {
            Ok (value ) => write ! (f , "{}: {:?}," , "days_to_return" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "days_to_return" , e . code () ) ? , 
        }
        match self . mode_to_return () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mode_to_return" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mode_to_return" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GetWeeklyScheduleRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GetWeeklyScheduleRequest" ) ;
        match self . days_to_return () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "days_to_return" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "days_to_return" , e . code () ) , 
        }
        match self . mode_to_return () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mode_to_return" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mode_to_return" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddThermostatSuggestionResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddThermostatSuggestionResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn unique_id (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddThermostatSuggestionResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddThermostatSuggestionResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddThermostatSuggestionResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddThermostatSuggestionResponse" ) ? ;
        match self . unique_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "unique_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddThermostatSuggestionResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddThermostatSuggestionResponse" ) ;
        match self . unique_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "unique_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetActiveScheduleRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetActiveScheduleRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn schedule_handle (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetActiveScheduleRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetActiveScheduleRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetActiveScheduleRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetActiveScheduleRequestRequest" ) ? ;
        match self . schedule_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "schedule_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "schedule_handle" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetActiveScheduleRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetActiveScheduleRequestRequest" ) ;
        match self . schedule_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "schedule_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "schedule_handle" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetActivePresetRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetActivePresetRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn preset_handle (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetActivePresetRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetActivePresetRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetActivePresetRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetActivePresetRequestRequest" ) ? ;
        match self . preset_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetActivePresetRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetActivePresetRequestRequest" ) ;
        match self . preset_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddThermostatSuggestionRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddThermostatSuggestionRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn preset_handle (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn effective_time (& self ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn expiration_in_minutes (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddThermostatSuggestionRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddThermostatSuggestionRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddThermostatSuggestionRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddThermostatSuggestionRequest" ) ? ;
        match self . preset_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "preset_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) ? , 
        }
        match self . effective_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "effective_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "effective_time" , e . code () ) ? , 
        }
        match self . expiration_in_minutes () {
            Ok (value ) => write ! (f , "{}: {:?}," , "expiration_in_minutes" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "expiration_in_minutes" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddThermostatSuggestionRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddThermostatSuggestionRequest" ) ;
        match self . preset_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "preset_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "preset_handle" , e . code () ) , 
        }
        match self . effective_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "effective_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "effective_time" , e . code () ) , 
        }
        match self . expiration_in_minutes () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "expiration_in_minutes" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "expiration_in_minutes" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RemoveThermostatSuggestionRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RemoveThermostatSuggestionRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn unique_id (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RemoveThermostatSuggestionRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RemoveThermostatSuggestionRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RemoveThermostatSuggestionRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RemoveThermostatSuggestionRequest" ) ? ;
        match self . unique_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "unique_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RemoveThermostatSuggestionRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RemoveThermostatSuggestionRequest" ) ;
        match self . unique_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "unique_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "unique_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AtomicResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AtomicResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn status_code (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn attribute_status (& self ) -> Result < crate :: tlv :: TLVArray < '_ , AtomicAttributeStatusStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn timeout (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AtomicResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AtomicResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AtomicResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AtomicResponse" ) ? ;
        match self . status_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "status_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "status_code" , e . code () ) ? , 
        }
        match self . attribute_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attribute_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attribute_status" , e . code () ) ? , 
        }
        match self . timeout () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "timeout" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "timeout" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "timeout" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AtomicResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AtomicResponse" ) ;
        match self . status_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "status_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "status_code" , e . code () ) , 
        }
        match self . attribute_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attribute_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attribute_status" , e . code () ) , 
        }
        match self . timeout () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "timeout" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "timeout" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "timeout" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AtomicRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AtomicRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn request_type (& self ) -> Result < AtomicRequestTypeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn attribute_requests (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: im :: AttrId > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn timeout (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AtomicRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AtomicRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AtomicRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AtomicRequestRequest" ) ? ;
        match self . request_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "request_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "request_type" , e . code () ) ? , 
        }
        match self . attribute_requests () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attribute_requests" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attribute_requests" , e . code () ) ? , 
        }
        match self . timeout () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "timeout" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "timeout" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "timeout" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AtomicRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AtomicRequestRequest" ) ;
        match self . request_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "request_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "request_type" , e . code () ) , 
        }
        match self . attribute_requests () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attribute_requests" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attribute_requests" , e . code () ) , 
        }
        match self . timeout () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "timeout" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "timeout" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "timeout" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct ScheduleTransitionStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ScheduleTransitionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn day_of_week (mut self , value : ScheduleDayOfWeekBitmap ) -> Result < ScheduleTransitionStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "dayOfWeek" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < ScheduleTransitionStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < ScheduleTransitionStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn system_mode (mut self , value : Option < SystemModeEnum > ) -> Result < ScheduleTransitionStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "systemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cooling_setpoint (mut self , value : Option < i16 > ) -> Result < ScheduleTransitionStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "coolingSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn heating_setpoint (mut self , value : Option < i16 > ) -> Result < ScheduleTransitionStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "heatingSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTransitionStructBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ScheduleTransitionStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleTransitionStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ScheduleTransitionStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleTransitionStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ScheduleTransitionStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleTransitionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScheduleTransitionStructArrayBuilder < P > (P ) ;
impl < P > ScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ScheduleTransitionStructBuilder < ScheduleTransitionStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScheduleTransitionStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ScheduleTransitionStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleTransitionStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ScheduleTransitionStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleTransitionStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScheduleStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ScheduleStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn schedule_handle (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < ScheduleStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "scheduleHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ScheduleStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn system_mode (mut self , value : SystemModeEnum ) -> Result < ScheduleStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "systemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ScheduleStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ScheduleStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ScheduleStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < ScheduleStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ScheduleStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transitions (self ) -> Result < ScheduleTransitionStructArrayBuilder < ScheduleStructBuilder < P , 5usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScheduleStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (4 ) , ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn built_in (mut self , value : crate :: tlv :: Nullable < bool > ) -> Result < ScheduleStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "builtIn" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ScheduleStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleStructBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ScheduleStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ScheduleStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ScheduleStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScheduleStructArrayBuilder < P > (P ) ;
impl < P > ScheduleStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ScheduleStructBuilder < ScheduleStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScheduleStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ScheduleStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ScheduleStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ScheduleStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PresetStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PresetStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < PresetStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_scenario (mut self , value : PresetScenarioEnum ) -> Result < PresetStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetScenario" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : Option < crate :: tlv :: Nullable < crate :: tlv :: Utf8Str < '_ > > > ) -> Result < PresetStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cooling_setpoint (mut self , value : Option < i16 > ) -> Result < PresetStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "coolingSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn heating_setpoint (mut self , value : Option < i16 > ) -> Result < PresetStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "heatingSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn built_in (mut self , value : crate :: tlv :: Nullable < bool > ) -> Result < PresetStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "builtIn" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (PresetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetStructBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PresetStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PresetStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PresetStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PresetStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PresetStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PresetStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PresetStructArrayBuilder < P > (P ) ;
impl < P > PresetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PresetStructBuilder < PresetStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PresetStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PresetStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PresetStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PresetStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PresetStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PresetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PresetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PresetTypeStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PresetTypeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PresetTypeStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_scenario (mut self , value : PresetScenarioEnum ) -> Result < PresetTypeStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetScenario" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PresetTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetTypeStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn number_of_presets (mut self , value : u8 ) -> Result < PresetTypeStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "numberOfPresets" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PresetTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetTypeStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_type_features (mut self , value : PresetTypeFeaturesBitmap ) -> Result < PresetTypeStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetTypeFeatures" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PresetTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PresetTypeStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PresetTypeStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PresetTypeStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PresetTypeStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PresetTypeStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PresetTypeStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PresetTypeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PresetTypeStructArrayBuilder < P > (P ) ;
impl < P > PresetTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PresetTypeStructBuilder < PresetTypeStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PresetTypeStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PresetTypeStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PresetTypeStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PresetTypeStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PresetTypeStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PresetTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PresetTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScheduleTypeStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ScheduleTypeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ScheduleTypeStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn system_mode (mut self , value : SystemModeEnum ) -> Result < ScheduleTypeStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "systemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ScheduleTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTypeStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn number_of_schedules (mut self , value : u8 ) -> Result < ScheduleTypeStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "numberOfSchedules" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ScheduleTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTypeStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn schedule_type_features (mut self , value : ScheduleTypeFeaturesBitmap ) -> Result < ScheduleTypeStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "scheduleTypeFeatures" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ScheduleTypeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScheduleTypeStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ScheduleTypeStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleTypeStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ScheduleTypeStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleTypeStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ScheduleTypeStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleTypeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScheduleTypeStructArrayBuilder < P > (P ) ;
impl < P > ScheduleTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ScheduleTypeStructBuilder < ScheduleTypeStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScheduleTypeStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ScheduleTypeStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScheduleTypeStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ScheduleTypeStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScheduleTypeStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ScheduleTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScheduleTypeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ThermostatSuggestionStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ThermostatSuggestionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ThermostatSuggestionStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn unique_id (mut self , value : u8 ) -> Result < ThermostatSuggestionStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "uniqueID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ThermostatSuggestionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThermostatSuggestionStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < ThermostatSuggestionStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ThermostatSuggestionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThermostatSuggestionStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn effective_time (mut self , value : u32 ) -> Result < ThermostatSuggestionStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "effectiveTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ThermostatSuggestionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThermostatSuggestionStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn expiration_time (mut self , value : u32 ) -> Result < ThermostatSuggestionStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "expirationTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ThermostatSuggestionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThermostatSuggestionStructBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ThermostatSuggestionStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ThermostatSuggestionStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ThermostatSuggestionStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ThermostatSuggestionStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ThermostatSuggestionStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ThermostatSuggestionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ThermostatSuggestionStructArrayBuilder < P > (P ) ;
impl < P > ThermostatSuggestionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ThermostatSuggestionStructBuilder < ThermostatSuggestionStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ThermostatSuggestionStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ThermostatSuggestionStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ThermostatSuggestionStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ThermostatSuggestionStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ThermostatSuggestionStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ThermostatSuggestionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ThermostatSuggestionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WeeklyScheduleTransitionStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > WeeklyScheduleTransitionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > WeeklyScheduleTransitionStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < WeeklyScheduleTransitionStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (WeeklyScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WeeklyScheduleTransitionStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn heat_setpoint (mut self , value : crate :: tlv :: Nullable < i16 > ) -> Result < WeeklyScheduleTransitionStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "heatSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (WeeklyScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WeeklyScheduleTransitionStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cool_setpoint (mut self , value : crate :: tlv :: Nullable < i16 > ) -> Result < WeeklyScheduleTransitionStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "coolSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (WeeklyScheduleTransitionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WeeklyScheduleTransitionStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for WeeklyScheduleTransitionStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WeeklyScheduleTransitionStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for WeeklyScheduleTransitionStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WeeklyScheduleTransitionStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for WeeklyScheduleTransitionStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WeeklyScheduleTransitionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WeeklyScheduleTransitionStructArrayBuilder < P > (P ) ;
impl < P > WeeklyScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < WeeklyScheduleTransitionStructBuilder < WeeklyScheduleTransitionStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WeeklyScheduleTransitionStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for WeeklyScheduleTransitionStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WeeklyScheduleTransitionStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for WeeklyScheduleTransitionStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WeeklyScheduleTransitionStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for WeeklyScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WeeklyScheduleTransitionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetpointRaiseLowerRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetpointRaiseLowerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetpointRaiseLowerRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mode (mut self , value : SetpointRaiseLowerModeEnum ) -> Result < SetpointRaiseLowerRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "mode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetpointRaiseLowerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointRaiseLowerRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn amount (mut self , value : i8 ) -> Result < SetpointRaiseLowerRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "amount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetpointRaiseLowerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointRaiseLowerRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetpointRaiseLowerRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetpointRaiseLowerRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetpointRaiseLowerRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetpointRaiseLowerRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetpointRaiseLowerRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetpointRaiseLowerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetpointRaiseLowerRequestArrayBuilder < P > (P ) ;
impl < P > SetpointRaiseLowerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetpointRaiseLowerRequestBuilder < SetpointRaiseLowerRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetpointRaiseLowerRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetpointRaiseLowerRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetpointRaiseLowerRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetpointRaiseLowerRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetpointRaiseLowerRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetpointRaiseLowerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetpointRaiseLowerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetWeeklyScheduleResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GetWeeklyScheduleResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GetWeeklyScheduleResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn number_of_transitions_for_sequence (mut self , value : u8 ) -> Result < GetWeeklyScheduleResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "numberOfTransitionsForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GetWeeklyScheduleResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetWeeklyScheduleResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn day_of_week_for_sequence (mut self , value : ScheduleDayOfWeekBitmap ) -> Result < GetWeeklyScheduleResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "dayOfWeekForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GetWeeklyScheduleResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetWeeklyScheduleResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mode_for_sequence (mut self , value : ScheduleModeBitmap ) -> Result < GetWeeklyScheduleResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "modeForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (GetWeeklyScheduleResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetWeeklyScheduleResponseBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transitions (self ) -> Result < WeeklyScheduleTransitionStructArrayBuilder < GetWeeklyScheduleResponseBuilder < P , 4usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetWeeklyScheduleResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > GetWeeklyScheduleResponseBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GetWeeklyScheduleResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GetWeeklyScheduleResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GetWeeklyScheduleResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetWeeklyScheduleResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetWeeklyScheduleResponseArrayBuilder < P > (P ) ;
impl < P > GetWeeklyScheduleResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GetWeeklyScheduleResponseBuilder < GetWeeklyScheduleResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetWeeklyScheduleResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GetWeeklyScheduleResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GetWeeklyScheduleResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GetWeeklyScheduleResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetWeeklyScheduleResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetWeeklyScheduleRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetWeeklyScheduleRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetWeeklyScheduleRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn number_of_transitions_for_sequence (mut self , value : u8 ) -> Result < SetWeeklyScheduleRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "numberOfTransitionsForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetWeeklyScheduleRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetWeeklyScheduleRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn day_of_week_for_sequence (mut self , value : ScheduleDayOfWeekBitmap ) -> Result < SetWeeklyScheduleRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "dayOfWeekForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetWeeklyScheduleRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetWeeklyScheduleRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mode_for_sequence (mut self , value : ScheduleModeBitmap ) -> Result < SetWeeklyScheduleRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "modeForSequence" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SetWeeklyScheduleRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetWeeklyScheduleRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transitions (self ) -> Result < WeeklyScheduleTransitionStructArrayBuilder < SetWeeklyScheduleRequestBuilder < P , 4usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetWeeklyScheduleRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > SetWeeklyScheduleRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetWeeklyScheduleRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetWeeklyScheduleRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetWeeklyScheduleRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetWeeklyScheduleRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetWeeklyScheduleRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetWeeklyScheduleRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetWeeklyScheduleRequestArrayBuilder < P > (P ) ;
impl < P > SetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetWeeklyScheduleRequestBuilder < SetWeeklyScheduleRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetWeeklyScheduleRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetWeeklyScheduleRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetWeeklyScheduleRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetWeeklyScheduleRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetWeeklyScheduleRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GetWeeklyScheduleRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GetWeeklyScheduleRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn days_to_return (mut self , value : ScheduleDayOfWeekBitmap ) -> Result < GetWeeklyScheduleRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "daysToReturn" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GetWeeklyScheduleRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetWeeklyScheduleRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mode_to_return (mut self , value : ScheduleModeBitmap ) -> Result < GetWeeklyScheduleRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "modeToReturn" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GetWeeklyScheduleRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetWeeklyScheduleRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GetWeeklyScheduleRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GetWeeklyScheduleRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GetWeeklyScheduleRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetWeeklyScheduleRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetWeeklyScheduleRequestArrayBuilder < P > (P ) ;
impl < P > GetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GetWeeklyScheduleRequestBuilder < GetWeeklyScheduleRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetWeeklyScheduleRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GetWeeklyScheduleRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetWeeklyScheduleRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetWeeklyScheduleRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddThermostatSuggestionResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddThermostatSuggestionResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddThermostatSuggestionResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn unique_id (mut self , value : u8 ) -> Result < AddThermostatSuggestionResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "uniqueID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddThermostatSuggestionResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddThermostatSuggestionResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddThermostatSuggestionResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddThermostatSuggestionResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddThermostatSuggestionResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddThermostatSuggestionResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddThermostatSuggestionResponseArrayBuilder < P > (P ) ;
impl < P > AddThermostatSuggestionResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddThermostatSuggestionResponseBuilder < AddThermostatSuggestionResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddThermostatSuggestionResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddThermostatSuggestionResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddThermostatSuggestionResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddThermostatSuggestionResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddThermostatSuggestionResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetActiveScheduleRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetActiveScheduleRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetActiveScheduleRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn schedule_handle (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < SetActiveScheduleRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "scheduleHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetActiveScheduleRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetActiveScheduleRequestRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetActiveScheduleRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetActiveScheduleRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetActiveScheduleRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetActiveScheduleRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetActiveScheduleRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetActiveScheduleRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetActiveScheduleRequestRequestArrayBuilder < P > (P ) ;
impl < P > SetActiveScheduleRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetActiveScheduleRequestRequestBuilder < SetActiveScheduleRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetActiveScheduleRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetActiveScheduleRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetActiveScheduleRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetActiveScheduleRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetActiveScheduleRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetActiveScheduleRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetActiveScheduleRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetActivePresetRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetActivePresetRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetActivePresetRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < SetActivePresetRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetActivePresetRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetActivePresetRequestRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetActivePresetRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetActivePresetRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetActivePresetRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetActivePresetRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetActivePresetRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetActivePresetRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetActivePresetRequestRequestArrayBuilder < P > (P ) ;
impl < P > SetActivePresetRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetActivePresetRequestRequestBuilder < SetActivePresetRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetActivePresetRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetActivePresetRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetActivePresetRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetActivePresetRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetActivePresetRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetActivePresetRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetActivePresetRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddThermostatSuggestionRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddThermostatSuggestionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddThermostatSuggestionRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn preset_handle (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddThermostatSuggestionRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "presetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddThermostatSuggestionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddThermostatSuggestionRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn effective_time (mut self , value : crate :: tlv :: Nullable < u32 > ) -> Result < AddThermostatSuggestionRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "effectiveTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AddThermostatSuggestionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddThermostatSuggestionRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn expiration_in_minutes (mut self , value : u16 ) -> Result < AddThermostatSuggestionRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "expirationInMinutes" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AddThermostatSuggestionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddThermostatSuggestionRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddThermostatSuggestionRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddThermostatSuggestionRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddThermostatSuggestionRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddThermostatSuggestionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddThermostatSuggestionRequestArrayBuilder < P > (P ) ;
impl < P > AddThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddThermostatSuggestionRequestBuilder < AddThermostatSuggestionRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddThermostatSuggestionRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddThermostatSuggestionRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddThermostatSuggestionRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddThermostatSuggestionRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveThermostatSuggestionRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RemoveThermostatSuggestionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RemoveThermostatSuggestionRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn unique_id (mut self , value : u8 ) -> Result < RemoveThermostatSuggestionRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "uniqueID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RemoveThermostatSuggestionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RemoveThermostatSuggestionRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RemoveThermostatSuggestionRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveThermostatSuggestionRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RemoveThermostatSuggestionRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveThermostatSuggestionRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RemoveThermostatSuggestionRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveThermostatSuggestionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveThermostatSuggestionRequestArrayBuilder < P > (P ) ;
impl < P > RemoveThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RemoveThermostatSuggestionRequestBuilder < RemoveThermostatSuggestionRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RemoveThermostatSuggestionRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RemoveThermostatSuggestionRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveThermostatSuggestionRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RemoveThermostatSuggestionRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveThermostatSuggestionRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RemoveThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveThermostatSuggestionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AtomicResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AtomicResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AtomicResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn status_code (mut self , value : u8 ) -> Result < AtomicResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "statusCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AtomicResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AtomicResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attribute_status (self ) -> Result < AtomicAttributeStatusStructArrayBuilder < AtomicResponseBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AtomicResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > AtomicResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn timeout (mut self , value : Option < u16 > ) -> Result < AtomicResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "timeout" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AtomicResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AtomicResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AtomicResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AtomicResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AtomicResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AtomicResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AtomicResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AtomicResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AtomicResponseArrayBuilder < P > (P ) ;
impl < P > AtomicResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AtomicResponseBuilder < AtomicResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AtomicResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AtomicResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AtomicResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AtomicResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AtomicResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AtomicResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AtomicResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AtomicRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AtomicRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AtomicRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn request_type (mut self , value : AtomicRequestTypeEnum ) -> Result < AtomicRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "requestType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AtomicRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AtomicRequestRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attribute_requests (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < AtomicRequestRequestBuilder < P , 2usize > , crate :: im :: AttrId > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AtomicRequestRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > AtomicRequestRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn timeout (mut self , value : Option < u16 > ) -> Result < AtomicRequestRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "timeout" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AtomicRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AtomicRequestRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AtomicRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AtomicRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AtomicRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AtomicRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AtomicRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AtomicRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AtomicRequestRequestArrayBuilder < P > (P ) ;
impl < P > AtomicRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AtomicRequestRequestBuilder < AtomicRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AtomicRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AtomicRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AtomicRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AtomicRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AtomicRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AtomicRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AtomicRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SystemModeChangeTag {
    PreviousSystemMode = 0 , CurrentSystemMode = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LocalTemperatureChangeTag {
    CurrentLocalTemperature = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum OccupancyChangeTag {
    PreviousOccupancy = 0 , CurrentOccupancy = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetpointChangeTag {
    SystemMode = 0 , Occupancy = 1 , PreviousSetpoint = 2 , CurrentSetpoint = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RunningStateChangeTag {
    PreviousRunningState = 0 , CurrentRunningState = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RunningModeChangeTag {
    PreviousRunningMode = 0 , CurrentRunningMode = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ActiveScheduleChangeTag {
    PreviousScheduleHandle = 0 , CurrentScheduleHandle = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ActivePresetChangeTag {
    PreviousPresetHandle = 0 , CurrentPresetHandle = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SystemModeChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SystemModeChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_system_mode (& self ) -> Result < Option < SystemModeEnum > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_system_mode (& self ) -> Result < SystemModeEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SystemModeChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SystemModeChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SystemModeChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SystemModeChange" ) ? ;
        match self . previous_system_mode () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_system_mode" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_system_mode" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_system_mode" , e . code () ) ? , 
        }
        match self . current_system_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_system_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_system_mode" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SystemModeChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SystemModeChange" ) ;
        match self . previous_system_mode () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_system_mode" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_system_mode" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_system_mode" , e . code () ) , 
        }
        match self . current_system_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_system_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_system_mode" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LocalTemperatureChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LocalTemperatureChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current_local_temperature (& self ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LocalTemperatureChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LocalTemperatureChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LocalTemperatureChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LocalTemperatureChange" ) ? ;
        match self . current_local_temperature () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_local_temperature" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_local_temperature" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LocalTemperatureChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LocalTemperatureChange" ) ;
        match self . current_local_temperature () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_local_temperature" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_local_temperature" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct OccupancyChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > OccupancyChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_occupancy (& self ) -> Result < Option < OccupancyBitmap > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_occupancy (& self ) -> Result < OccupancyBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for OccupancyChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for OccupancyChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for OccupancyChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "OccupancyChange" ) ? ;
        match self . previous_occupancy () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_occupancy" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_occupancy" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_occupancy" , e . code () ) ? , 
        }
        match self . current_occupancy () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_occupancy" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_occupancy" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for OccupancyChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "OccupancyChange" ) ;
        match self . previous_occupancy () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_occupancy" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_occupancy" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_occupancy" , e . code () ) , 
        }
        match self . current_occupancy () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_occupancy" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_occupancy" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetpointChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetpointChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn system_mode (& self ) -> Result < SystemModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn occupancy (& self ) -> Result < Option < OccupancyBitmap > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn previous_setpoint (& self ) -> Result < Option < i16 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn current_setpoint (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetpointChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetpointChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetpointChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetpointChange" ) ? ;
        match self . system_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "system_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) ? , 
        }
        match self . occupancy () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "occupancy" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "occupancy" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "occupancy" , e . code () ) ? , 
        }
        match self . previous_setpoint () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_setpoint" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_setpoint" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_setpoint" , e . code () ) ? , 
        }
        match self . current_setpoint () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_setpoint" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_setpoint" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetpointChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetpointChange" ) ;
        match self . system_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "system_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "system_mode" , e . code () ) , 
        }
        match self . occupancy () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "occupancy" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "occupancy" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "occupancy" , e . code () ) , 
        }
        match self . previous_setpoint () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_setpoint" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_setpoint" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_setpoint" , e . code () ) , 
        }
        match self . current_setpoint () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_setpoint" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_setpoint" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RunningStateChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RunningStateChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_running_state (& self ) -> Result < Option < RelayStateBitmap > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_running_state (& self ) -> Result < RelayStateBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RunningStateChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RunningStateChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RunningStateChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RunningStateChange" ) ? ;
        match self . previous_running_state () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_running_state" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_running_state" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_running_state" , e . code () ) ? , 
        }
        match self . current_running_state () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_running_state" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_running_state" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RunningStateChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RunningStateChange" ) ;
        match self . previous_running_state () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_running_state" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_running_state" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_running_state" , e . code () ) , 
        }
        match self . current_running_state () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_running_state" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_running_state" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RunningModeChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RunningModeChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_running_mode (& self ) -> Result < Option < ThermostatRunningModeEnum > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_running_mode (& self ) -> Result < ThermostatRunningModeEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RunningModeChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RunningModeChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RunningModeChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RunningModeChange" ) ? ;
        match self . previous_running_mode () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_running_mode" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_running_mode" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_running_mode" , e . code () ) ? , 
        }
        match self . current_running_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_running_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_running_mode" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RunningModeChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RunningModeChange" ) ;
        match self . previous_running_mode () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_running_mode" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_running_mode" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_running_mode" , e . code () ) , 
        }
        match self . current_running_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_running_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_running_mode" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ActiveScheduleChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ActiveScheduleChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_schedule_handle (& self ) -> Result < Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_schedule_handle (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ActiveScheduleChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ActiveScheduleChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ActiveScheduleChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ActiveScheduleChange" ) ? ;
        match self . previous_schedule_handle () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_schedule_handle" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_schedule_handle" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_schedule_handle" , e . code () ) ? , 
        }
        match self . current_schedule_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_schedule_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_schedule_handle" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ActiveScheduleChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ActiveScheduleChange" ) ;
        match self . previous_schedule_handle () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_schedule_handle" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_schedule_handle" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_schedule_handle" , e . code () ) , 
        }
        match self . current_schedule_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_schedule_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_schedule_handle" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ActivePresetChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ActivePresetChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_preset_handle (& self ) -> Result < Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn current_preset_handle (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ActivePresetChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ActivePresetChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ActivePresetChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ActivePresetChange" ) ? ;
        match self . previous_preset_handle () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_preset_handle" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_preset_handle" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_preset_handle" , e . code () ) ? , 
        }
        match self . current_preset_handle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_preset_handle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_preset_handle" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ActivePresetChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ActivePresetChange" ) ;
        match self . previous_preset_handle () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_preset_handle" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_preset_handle" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_preset_handle" , e . code () ) , 
        }
        match self . current_preset_handle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_preset_handle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_preset_handle" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct SystemModeChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SystemModeChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SystemModeChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_system_mode (mut self , value : Option < SystemModeEnum > ) -> Result < SystemModeChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousSystemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SystemModeChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SystemModeChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_system_mode (mut self , value : SystemModeEnum ) -> Result < SystemModeChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentSystemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SystemModeChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SystemModeChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SystemModeChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SystemModeChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SystemModeChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SystemModeChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SystemModeChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SystemModeChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SystemModeChangeArrayBuilder < P > (P ) ;
impl < P > SystemModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SystemModeChangeBuilder < SystemModeChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SystemModeChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SystemModeChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SystemModeChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SystemModeChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SystemModeChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SystemModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SystemModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LocalTemperatureChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LocalTemperatureChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LocalTemperatureChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_local_temperature (mut self , value : crate :: tlv :: Nullable < i16 > ) -> Result < LocalTemperatureChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentLocalTemperature" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LocalTemperatureChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > LocalTemperatureChangeBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LocalTemperatureChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LocalTemperatureChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LocalTemperatureChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LocalTemperatureChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LocalTemperatureChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LocalTemperatureChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LocalTemperatureChangeArrayBuilder < P > (P ) ;
impl < P > LocalTemperatureChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LocalTemperatureChangeBuilder < LocalTemperatureChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LocalTemperatureChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LocalTemperatureChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LocalTemperatureChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LocalTemperatureChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LocalTemperatureChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LocalTemperatureChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LocalTemperatureChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct OccupancyChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > OccupancyChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > OccupancyChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_occupancy (mut self , value : Option < OccupancyBitmap > ) -> Result < OccupancyChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousOccupancy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (OccupancyChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > OccupancyChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_occupancy (mut self , value : OccupancyBitmap ) -> Result < OccupancyChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentOccupancy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (OccupancyChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > OccupancyChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for OccupancyChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "OccupancyChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for OccupancyChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "OccupancyChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for OccupancyChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for OccupancyChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct OccupancyChangeArrayBuilder < P > (P ) ;
impl < P > OccupancyChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < OccupancyChangeBuilder < OccupancyChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (OccupancyChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for OccupancyChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "OccupancyChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for OccupancyChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "OccupancyChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for OccupancyChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for OccupancyChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetpointChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetpointChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetpointChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn system_mode (mut self , value : SystemModeEnum ) -> Result < SetpointChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "systemMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetpointChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn occupancy (mut self , value : Option < OccupancyBitmap > ) -> Result < SetpointChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "occupancy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetpointChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointChangeBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_setpoint (mut self , value : Option < i16 > ) -> Result < SetpointChangeBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SetpointChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointChangeBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_setpoint (mut self , value : i16 ) -> Result < SetpointChangeBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentSetpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (SetpointChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetpointChangeBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetpointChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetpointChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetpointChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetpointChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetpointChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetpointChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetpointChangeArrayBuilder < P > (P ) ;
impl < P > SetpointChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetpointChangeBuilder < SetpointChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetpointChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetpointChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetpointChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetpointChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetpointChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetpointChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetpointChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RunningStateChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RunningStateChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RunningStateChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_running_state (mut self , value : Option < RelayStateBitmap > ) -> Result < RunningStateChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousRunningState" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RunningStateChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > RunningStateChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_running_state (mut self , value : RelayStateBitmap ) -> Result < RunningStateChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentRunningState" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (RunningStateChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > RunningStateChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RunningStateChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RunningStateChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RunningStateChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RunningStateChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RunningStateChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RunningStateChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RunningStateChangeArrayBuilder < P > (P ) ;
impl < P > RunningStateChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RunningStateChangeBuilder < RunningStateChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RunningStateChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RunningStateChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RunningStateChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RunningStateChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RunningStateChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RunningStateChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RunningStateChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RunningModeChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RunningModeChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RunningModeChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_running_mode (mut self , value : Option < ThermostatRunningModeEnum > ) -> Result < RunningModeChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousRunningMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RunningModeChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > RunningModeChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_running_mode (mut self , value : ThermostatRunningModeEnum ) -> Result < RunningModeChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentRunningMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (RunningModeChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > RunningModeChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RunningModeChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RunningModeChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RunningModeChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RunningModeChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RunningModeChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RunningModeChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RunningModeChangeArrayBuilder < P > (P ) ;
impl < P > RunningModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RunningModeChangeBuilder < RunningModeChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RunningModeChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RunningModeChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RunningModeChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RunningModeChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RunningModeChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RunningModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RunningModeChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ActiveScheduleChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ActiveScheduleChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ActiveScheduleChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_schedule_handle (mut self , value : Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > ) -> Result < ActiveScheduleChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousScheduleHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ActiveScheduleChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > ActiveScheduleChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_schedule_handle (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < ActiveScheduleChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentScheduleHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ActiveScheduleChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > ActiveScheduleChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ActiveScheduleChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ActiveScheduleChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ActiveScheduleChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ActiveScheduleChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ActiveScheduleChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ActiveScheduleChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ActiveScheduleChangeArrayBuilder < P > (P ) ;
impl < P > ActiveScheduleChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ActiveScheduleChangeBuilder < ActiveScheduleChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ActiveScheduleChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ActiveScheduleChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ActiveScheduleChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ActiveScheduleChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ActiveScheduleChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ActiveScheduleChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ActiveScheduleChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ActivePresetChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ActivePresetChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ActivePresetChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_preset_handle (mut self , value : Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > ) -> Result < ActivePresetChangeBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousPresetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ActivePresetChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > ActivePresetChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_preset_handle (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < ActivePresetChangeBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentPresetHandle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ActivePresetChangeBuilder (self . 0 ) ) 
    }
    
}
impl < P > ActivePresetChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ActivePresetChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ActivePresetChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ActivePresetChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ActivePresetChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ActivePresetChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ActivePresetChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ActivePresetChangeArrayBuilder < P > (P ) ;
impl < P > ActivePresetChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ActivePresetChangeBuilder < ActivePresetChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ActivePresetChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ActivePresetChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ActivePresetChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ActivePresetChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ActivePresetChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ActivePresetChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ActivePresetChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl SystemModeChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SystemModeChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: SystemModeChange as _ , Self :: PRIORITY , | tw | {
            f (SystemModeChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SystemModeChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: SystemModeChange as _ , Self :: PRIORITY , | tw | {
            f (SystemModeChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl LocalTemperatureChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LocalTemperatureChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: LocalTemperatureChange as _ , Self :: PRIORITY , | tw | {
            f (LocalTemperatureChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LocalTemperatureChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: LocalTemperatureChange as _ , Self :: PRIORITY , | tw | {
            f (LocalTemperatureChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl OccupancyChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (OccupancyChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: OccupancyChange as _ , Self :: PRIORITY , | tw | {
            f (OccupancyChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (OccupancyChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: OccupancyChange as _ , Self :: PRIORITY , | tw | {
            f (OccupancyChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl SetpointChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SetpointChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: SetpointChange as _ , Self :: PRIORITY , | tw | {
            f (SetpointChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SetpointChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: SetpointChange as _ , Self :: PRIORITY , | tw | {
            f (SetpointChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl RunningStateChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RunningStateChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: RunningStateChange as _ , Self :: PRIORITY , | tw | {
            f (RunningStateChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RunningStateChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: RunningStateChange as _ , Self :: PRIORITY , | tw | {
            f (RunningStateChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl RunningModeChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RunningModeChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: RunningModeChange as _ , Self :: PRIORITY , | tw | {
            f (RunningModeChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RunningModeChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: RunningModeChange as _ , Self :: PRIORITY , | tw | {
            f (RunningModeChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl ActiveScheduleChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ActiveScheduleChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: ActiveScheduleChange as _ , Self :: PRIORITY , | tw | {
            f (ActiveScheduleChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ActiveScheduleChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: ActiveScheduleChange as _ , Self :: PRIORITY , | tw | {
            f (ActiveScheduleChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl ActivePresetChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ActivePresetChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 513 , EventId :: ActivePresetChange as _ , Self :: PRIORITY , | tw | {
            f (ActivePresetChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ActivePresetChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: ActivePresetChange as _ , Self :: PRIORITY , | tw | {
            f (ActivePresetChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    LocalTemperature = 0 , OutdoorTemperature = 1 , Occupancy = 2 , AbsMinHeatSetpointLimit = 3 , AbsMaxHeatSetpointLimit = 4 , AbsMinCoolSetpointLimit = 5 , AbsMaxCoolSetpointLimit = 6 , PICoolingDemand = 7 , PIHeatingDemand = 8 , HVACSystemTypeConfiguration = 9 , LocalTemperatureCalibration = 16 , OccupiedCoolingSetpoint = 17 , OccupiedHeatingSetpoint = 18 , UnoccupiedCoolingSetpoint = 19 , UnoccupiedHeatingSetpoint = 20 , MinHeatSetpointLimit = 21 , MaxHeatSetpointLimit = 22 , MinCoolSetpointLimit = 23 , MaxCoolSetpointLimit = 24 , MinSetpointDeadBand = 25 , RemoteSensing = 26 , ControlSequenceOfOperation = 27 , SystemMode = 28 , ThermostatRunningMode = 30 , StartOfWeek = 32 , NumberOfWeeklyTransitions = 33 , NumberOfDailyTransitions = 34 , TemperatureSetpointHold = 35 , TemperatureSetpointHoldDuration = 36 , ThermostatProgrammingOperationMode = 37 , ThermostatRunningState = 41 , SetpointChangeSource = 48 , SetpointChangeAmount = 49 , SetpointChangeSourceTimestamp = 50 , OccupiedSetback = 52 , OccupiedSetbackMin = 53 , OccupiedSetbackMax = 54 , UnoccupiedSetback = 55 , UnoccupiedSetbackMin = 56 , UnoccupiedSetbackMax = 57 , EmergencyHeatDelta = 58 , ACType = 64 , ACCapacity = 65 , ACRefrigerantType = 66 , ACCompressorType = 67 , ACErrorCode = 68 , ACLouverPosition = 69 , ACCoilTemperature = 70 , ACCapacityformat = 71 , PresetTypes = 72 , ScheduleTypes = 73 , NumberOfPresets = 74 , NumberOfSchedules = 75 , NumberOfScheduleTransitions = 76 , NumberOfScheduleTransitionPerDay = 77 , ActivePresetHandle = 78 , ActiveScheduleHandle = 79 , Presets = 80 , Schedules = 81 , SetpointHoldExpiryTimestamp = 82 , MaxThermostatSuggestions = 83 , ThermostatSuggestions = 84 , CurrentThermostatSuggestion = 85 , ThermostatSuggestionNotFollowingReason = 86 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: LocalTemperature => write ! (f , "{}(0x{:02x})" , "LocalTemperature" , AttributeId :: LocalTemperature as u32 ) ? , AttributeId :: OutdoorTemperature => write ! (f , "{}(0x{:02x})" , "OutdoorTemperature" , AttributeId :: OutdoorTemperature as u32 ) ? , AttributeId :: Occupancy => write ! (f , "{}(0x{:02x})" , "Occupancy" , AttributeId :: Occupancy as u32 ) ? , AttributeId :: AbsMinHeatSetpointLimit => write ! (f , "{}(0x{:02x})" , "AbsMinHeatSetpointLimit" , AttributeId :: AbsMinHeatSetpointLimit as u32 ) ? , AttributeId :: AbsMaxHeatSetpointLimit => write ! (f , "{}(0x{:02x})" , "AbsMaxHeatSetpointLimit" , AttributeId :: AbsMaxHeatSetpointLimit as u32 ) ? , AttributeId :: AbsMinCoolSetpointLimit => write ! (f , "{}(0x{:02x})" , "AbsMinCoolSetpointLimit" , AttributeId :: AbsMinCoolSetpointLimit as u32 ) ? , AttributeId :: AbsMaxCoolSetpointLimit => write ! (f , "{}(0x{:02x})" , "AbsMaxCoolSetpointLimit" , AttributeId :: AbsMaxCoolSetpointLimit as u32 ) ? , AttributeId :: PICoolingDemand => write ! (f , "{}(0x{:02x})" , "PICoolingDemand" , AttributeId :: PICoolingDemand as u32 ) ? , AttributeId :: PIHeatingDemand => write ! (f , "{}(0x{:02x})" , "PIHeatingDemand" , AttributeId :: PIHeatingDemand as u32 ) ? , AttributeId :: HVACSystemTypeConfiguration => write ! (f , "{}(0x{:02x})" , "HVACSystemTypeConfiguration" , AttributeId :: HVACSystemTypeConfiguration as u32 ) ? , AttributeId :: LocalTemperatureCalibration => write ! (f , "{}(0x{:02x})" , "LocalTemperatureCalibration" , AttributeId :: LocalTemperatureCalibration as u32 ) ? , AttributeId :: OccupiedCoolingSetpoint => write ! (f , "{}(0x{:02x})" , "OccupiedCoolingSetpoint" , AttributeId :: OccupiedCoolingSetpoint as u32 ) ? , AttributeId :: OccupiedHeatingSetpoint => write ! (f , "{}(0x{:02x})" , "OccupiedHeatingSetpoint" , AttributeId :: OccupiedHeatingSetpoint as u32 ) ? , AttributeId :: UnoccupiedCoolingSetpoint => write ! (f , "{}(0x{:02x})" , "UnoccupiedCoolingSetpoint" , AttributeId :: UnoccupiedCoolingSetpoint as u32 ) ? , AttributeId :: UnoccupiedHeatingSetpoint => write ! (f , "{}(0x{:02x})" , "UnoccupiedHeatingSetpoint" , AttributeId :: UnoccupiedHeatingSetpoint as u32 ) ? , AttributeId :: MinHeatSetpointLimit => write ! (f , "{}(0x{:02x})" , "MinHeatSetpointLimit" , AttributeId :: MinHeatSetpointLimit as u32 ) ? , AttributeId :: MaxHeatSetpointLimit => write ! (f , "{}(0x{:02x})" , "MaxHeatSetpointLimit" , AttributeId :: MaxHeatSetpointLimit as u32 ) ? , AttributeId :: MinCoolSetpointLimit => write ! (f , "{}(0x{:02x})" , "MinCoolSetpointLimit" , AttributeId :: MinCoolSetpointLimit as u32 ) ? , AttributeId :: MaxCoolSetpointLimit => write ! (f , "{}(0x{:02x})" , "MaxCoolSetpointLimit" , AttributeId :: MaxCoolSetpointLimit as u32 ) ? , AttributeId :: MinSetpointDeadBand => write ! (f , "{}(0x{:02x})" , "MinSetpointDeadBand" , AttributeId :: MinSetpointDeadBand as u32 ) ? , AttributeId :: RemoteSensing => write ! (f , "{}(0x{:02x})" , "RemoteSensing" , AttributeId :: RemoteSensing as u32 ) ? , AttributeId :: ControlSequenceOfOperation => write ! (f , "{}(0x{:02x})" , "ControlSequenceOfOperation" , AttributeId :: ControlSequenceOfOperation as u32 ) ? , AttributeId :: SystemMode => write ! (f , "{}(0x{:02x})" , "SystemMode" , AttributeId :: SystemMode as u32 ) ? , AttributeId :: ThermostatRunningMode => write ! (f , "{}(0x{:02x})" , "ThermostatRunningMode" , AttributeId :: ThermostatRunningMode as u32 ) ? , AttributeId :: StartOfWeek => write ! (f , "{}(0x{:02x})" , "StartOfWeek" , AttributeId :: StartOfWeek as u32 ) ? , AttributeId :: NumberOfWeeklyTransitions => write ! (f , "{}(0x{:02x})" , "NumberOfWeeklyTransitions" , AttributeId :: NumberOfWeeklyTransitions as u32 ) ? , AttributeId :: NumberOfDailyTransitions => write ! (f , "{}(0x{:02x})" , "NumberOfDailyTransitions" , AttributeId :: NumberOfDailyTransitions as u32 ) ? , AttributeId :: TemperatureSetpointHold => write ! (f , "{}(0x{:02x})" , "TemperatureSetpointHold" , AttributeId :: TemperatureSetpointHold as u32 ) ? , AttributeId :: TemperatureSetpointHoldDuration => write ! (f , "{}(0x{:02x})" , "TemperatureSetpointHoldDuration" , AttributeId :: TemperatureSetpointHoldDuration as u32 ) ? , AttributeId :: ThermostatProgrammingOperationMode => write ! (f , "{}(0x{:02x})" , "ThermostatProgrammingOperationMode" , AttributeId :: ThermostatProgrammingOperationMode as u32 ) ? , AttributeId :: ThermostatRunningState => write ! (f , "{}(0x{:02x})" , "ThermostatRunningState" , AttributeId :: ThermostatRunningState as u32 ) ? , AttributeId :: SetpointChangeSource => write ! (f , "{}(0x{:02x})" , "SetpointChangeSource" , AttributeId :: SetpointChangeSource as u32 ) ? , AttributeId :: SetpointChangeAmount => write ! (f , "{}(0x{:02x})" , "SetpointChangeAmount" , AttributeId :: SetpointChangeAmount as u32 ) ? , AttributeId :: SetpointChangeSourceTimestamp => write ! (f , "{}(0x{:02x})" , "SetpointChangeSourceTimestamp" , AttributeId :: SetpointChangeSourceTimestamp as u32 ) ? , AttributeId :: OccupiedSetback => write ! (f , "{}(0x{:02x})" , "OccupiedSetback" , AttributeId :: OccupiedSetback as u32 ) ? , AttributeId :: OccupiedSetbackMin => write ! (f , "{}(0x{:02x})" , "OccupiedSetbackMin" , AttributeId :: OccupiedSetbackMin as u32 ) ? , AttributeId :: OccupiedSetbackMax => write ! (f , "{}(0x{:02x})" , "OccupiedSetbackMax" , AttributeId :: OccupiedSetbackMax as u32 ) ? , AttributeId :: UnoccupiedSetback => write ! (f , "{}(0x{:02x})" , "UnoccupiedSetback" , AttributeId :: UnoccupiedSetback as u32 ) ? , AttributeId :: UnoccupiedSetbackMin => write ! (f , "{}(0x{:02x})" , "UnoccupiedSetbackMin" , AttributeId :: UnoccupiedSetbackMin as u32 ) ? , AttributeId :: UnoccupiedSetbackMax => write ! (f , "{}(0x{:02x})" , "UnoccupiedSetbackMax" , AttributeId :: UnoccupiedSetbackMax as u32 ) ? , AttributeId :: EmergencyHeatDelta => write ! (f , "{}(0x{:02x})" , "EmergencyHeatDelta" , AttributeId :: EmergencyHeatDelta as u32 ) ? , AttributeId :: ACType => write ! (f , "{}(0x{:02x})" , "ACType" , AttributeId :: ACType as u32 ) ? , AttributeId :: ACCapacity => write ! (f , "{}(0x{:02x})" , "ACCapacity" , AttributeId :: ACCapacity as u32 ) ? , AttributeId :: ACRefrigerantType => write ! (f , "{}(0x{:02x})" , "ACRefrigerantType" , AttributeId :: ACRefrigerantType as u32 ) ? , AttributeId :: ACCompressorType => write ! (f , "{}(0x{:02x})" , "ACCompressorType" , AttributeId :: ACCompressorType as u32 ) ? , AttributeId :: ACErrorCode => write ! (f , "{}(0x{:02x})" , "ACErrorCode" , AttributeId :: ACErrorCode as u32 ) ? , AttributeId :: ACLouverPosition => write ! (f , "{}(0x{:02x})" , "ACLouverPosition" , AttributeId :: ACLouverPosition as u32 ) ? , AttributeId :: ACCoilTemperature => write ! (f , "{}(0x{:02x})" , "ACCoilTemperature" , AttributeId :: ACCoilTemperature as u32 ) ? , AttributeId :: ACCapacityformat => write ! (f , "{}(0x{:02x})" , "ACCapacityformat" , AttributeId :: ACCapacityformat as u32 ) ? , AttributeId :: PresetTypes => write ! (f , "{}(0x{:02x})" , "PresetTypes" , AttributeId :: PresetTypes as u32 ) ? , AttributeId :: ScheduleTypes => write ! (f , "{}(0x{:02x})" , "ScheduleTypes" , AttributeId :: ScheduleTypes as u32 ) ? , AttributeId :: NumberOfPresets => write ! (f , "{}(0x{:02x})" , "NumberOfPresets" , AttributeId :: NumberOfPresets as u32 ) ? , AttributeId :: NumberOfSchedules => write ! (f , "{}(0x{:02x})" , "NumberOfSchedules" , AttributeId :: NumberOfSchedules as u32 ) ? , AttributeId :: NumberOfScheduleTransitions => write ! (f , "{}(0x{:02x})" , "NumberOfScheduleTransitions" , AttributeId :: NumberOfScheduleTransitions as u32 ) ? , AttributeId :: NumberOfScheduleTransitionPerDay => write ! (f , "{}(0x{:02x})" , "NumberOfScheduleTransitionPerDay" , AttributeId :: NumberOfScheduleTransitionPerDay as u32 ) ? , AttributeId :: ActivePresetHandle => write ! (f , "{}(0x{:02x})" , "ActivePresetHandle" , AttributeId :: ActivePresetHandle as u32 ) ? , AttributeId :: ActiveScheduleHandle => write ! (f , "{}(0x{:02x})" , "ActiveScheduleHandle" , AttributeId :: ActiveScheduleHandle as u32 ) ? , AttributeId :: Presets => write ! (f , "{}(0x{:02x})" , "Presets" , AttributeId :: Presets as u32 ) ? , AttributeId :: Schedules => write ! (f , "{}(0x{:02x})" , "Schedules" , AttributeId :: Schedules as u32 ) ? , AttributeId :: SetpointHoldExpiryTimestamp => write ! (f , "{}(0x{:02x})" , "SetpointHoldExpiryTimestamp" , AttributeId :: SetpointHoldExpiryTimestamp as u32 ) ? , AttributeId :: MaxThermostatSuggestions => write ! (f , "{}(0x{:02x})" , "MaxThermostatSuggestions" , AttributeId :: MaxThermostatSuggestions as u32 ) ? , AttributeId :: ThermostatSuggestions => write ! (f , "{}(0x{:02x})" , "ThermostatSuggestions" , AttributeId :: ThermostatSuggestions as u32 ) ? , AttributeId :: CurrentThermostatSuggestion => write ! (f , "{}(0x{:02x})" , "CurrentThermostatSuggestion" , AttributeId :: CurrentThermostatSuggestion as u32 ) ? , AttributeId :: ThermostatSuggestionNotFollowingReason => write ! (f , "{}(0x{:02x})" , "ThermostatSuggestionNotFollowingReason" , AttributeId :: ThermostatSuggestionNotFollowingReason as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: LocalTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocalTemperature" , AttributeId :: LocalTemperature as u32 ) , AttributeId :: OutdoorTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OutdoorTemperature" , AttributeId :: OutdoorTemperature as u32 ) , AttributeId :: Occupancy => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Occupancy" , AttributeId :: Occupancy as u32 ) , AttributeId :: AbsMinHeatSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AbsMinHeatSetpointLimit" , AttributeId :: AbsMinHeatSetpointLimit as u32 ) , AttributeId :: AbsMaxHeatSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AbsMaxHeatSetpointLimit" , AttributeId :: AbsMaxHeatSetpointLimit as u32 ) , AttributeId :: AbsMinCoolSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AbsMinCoolSetpointLimit" , AttributeId :: AbsMinCoolSetpointLimit as u32 ) , AttributeId :: AbsMaxCoolSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AbsMaxCoolSetpointLimit" , AttributeId :: AbsMaxCoolSetpointLimit as u32 ) , AttributeId :: PICoolingDemand => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PICoolingDemand" , AttributeId :: PICoolingDemand as u32 ) , AttributeId :: PIHeatingDemand => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PIHeatingDemand" , AttributeId :: PIHeatingDemand as u32 ) , AttributeId :: HVACSystemTypeConfiguration => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HVACSystemTypeConfiguration" , AttributeId :: HVACSystemTypeConfiguration as u32 ) , AttributeId :: LocalTemperatureCalibration => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocalTemperatureCalibration" , AttributeId :: LocalTemperatureCalibration as u32 ) , AttributeId :: OccupiedCoolingSetpoint => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupiedCoolingSetpoint" , AttributeId :: OccupiedCoolingSetpoint as u32 ) , AttributeId :: OccupiedHeatingSetpoint => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupiedHeatingSetpoint" , AttributeId :: OccupiedHeatingSetpoint as u32 ) , AttributeId :: UnoccupiedCoolingSetpoint => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UnoccupiedCoolingSetpoint" , AttributeId :: UnoccupiedCoolingSetpoint as u32 ) , AttributeId :: UnoccupiedHeatingSetpoint => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UnoccupiedHeatingSetpoint" , AttributeId :: UnoccupiedHeatingSetpoint as u32 ) , AttributeId :: MinHeatSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinHeatSetpointLimit" , AttributeId :: MinHeatSetpointLimit as u32 ) , AttributeId :: MaxHeatSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxHeatSetpointLimit" , AttributeId :: MaxHeatSetpointLimit as u32 ) , AttributeId :: MinCoolSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinCoolSetpointLimit" , AttributeId :: MinCoolSetpointLimit as u32 ) , AttributeId :: MaxCoolSetpointLimit => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxCoolSetpointLimit" , AttributeId :: MaxCoolSetpointLimit as u32 ) , AttributeId :: MinSetpointDeadBand => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinSetpointDeadBand" , AttributeId :: MinSetpointDeadBand as u32 ) , AttributeId :: RemoteSensing => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemoteSensing" , AttributeId :: RemoteSensing as u32 ) , AttributeId :: ControlSequenceOfOperation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ControlSequenceOfOperation" , AttributeId :: ControlSequenceOfOperation as u32 ) , AttributeId :: SystemMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SystemMode" , AttributeId :: SystemMode as u32 ) , AttributeId :: ThermostatRunningMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThermostatRunningMode" , AttributeId :: ThermostatRunningMode as u32 ) , AttributeId :: StartOfWeek => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StartOfWeek" , AttributeId :: StartOfWeek as u32 ) , AttributeId :: NumberOfWeeklyTransitions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfWeeklyTransitions" , AttributeId :: NumberOfWeeklyTransitions as u32 ) , AttributeId :: NumberOfDailyTransitions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfDailyTransitions" , AttributeId :: NumberOfDailyTransitions as u32 ) , AttributeId :: TemperatureSetpointHold => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TemperatureSetpointHold" , AttributeId :: TemperatureSetpointHold as u32 ) , AttributeId :: TemperatureSetpointHoldDuration => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TemperatureSetpointHoldDuration" , AttributeId :: TemperatureSetpointHoldDuration as u32 ) , AttributeId :: ThermostatProgrammingOperationMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThermostatProgrammingOperationMode" , AttributeId :: ThermostatProgrammingOperationMode as u32 ) , AttributeId :: ThermostatRunningState => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThermostatRunningState" , AttributeId :: ThermostatRunningState as u32 ) , AttributeId :: SetpointChangeSource => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointChangeSource" , AttributeId :: SetpointChangeSource as u32 ) , AttributeId :: SetpointChangeAmount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointChangeAmount" , AttributeId :: SetpointChangeAmount as u32 ) , AttributeId :: SetpointChangeSourceTimestamp => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointChangeSourceTimestamp" , AttributeId :: SetpointChangeSourceTimestamp as u32 ) , AttributeId :: OccupiedSetback => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupiedSetback" , AttributeId :: OccupiedSetback as u32 ) , AttributeId :: OccupiedSetbackMin => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupiedSetbackMin" , AttributeId :: OccupiedSetbackMin as u32 ) , AttributeId :: OccupiedSetbackMax => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupiedSetbackMax" , AttributeId :: OccupiedSetbackMax as u32 ) , AttributeId :: UnoccupiedSetback => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UnoccupiedSetback" , AttributeId :: UnoccupiedSetback as u32 ) , AttributeId :: UnoccupiedSetbackMin => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UnoccupiedSetbackMin" , AttributeId :: UnoccupiedSetbackMin as u32 ) , AttributeId :: UnoccupiedSetbackMax => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UnoccupiedSetbackMax" , AttributeId :: UnoccupiedSetbackMax as u32 ) , AttributeId :: EmergencyHeatDelta => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EmergencyHeatDelta" , AttributeId :: EmergencyHeatDelta as u32 ) , AttributeId :: ACType => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACType" , AttributeId :: ACType as u32 ) , AttributeId :: ACCapacity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACCapacity" , AttributeId :: ACCapacity as u32 ) , AttributeId :: ACRefrigerantType => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACRefrigerantType" , AttributeId :: ACRefrigerantType as u32 ) , AttributeId :: ACCompressorType => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACCompressorType" , AttributeId :: ACCompressorType as u32 ) , AttributeId :: ACErrorCode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACErrorCode" , AttributeId :: ACErrorCode as u32 ) , AttributeId :: ACLouverPosition => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACLouverPosition" , AttributeId :: ACLouverPosition as u32 ) , AttributeId :: ACCoilTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACCoilTemperature" , AttributeId :: ACCoilTemperature as u32 ) , AttributeId :: ACCapacityformat => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ACCapacityformat" , AttributeId :: ACCapacityformat as u32 ) , AttributeId :: PresetTypes => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PresetTypes" , AttributeId :: PresetTypes as u32 ) , AttributeId :: ScheduleTypes => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ScheduleTypes" , AttributeId :: ScheduleTypes as u32 ) , AttributeId :: NumberOfPresets => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfPresets" , AttributeId :: NumberOfPresets as u32 ) , AttributeId :: NumberOfSchedules => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfSchedules" , AttributeId :: NumberOfSchedules as u32 ) , AttributeId :: NumberOfScheduleTransitions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfScheduleTransitions" , AttributeId :: NumberOfScheduleTransitions as u32 ) , AttributeId :: NumberOfScheduleTransitionPerDay => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfScheduleTransitionPerDay" , AttributeId :: NumberOfScheduleTransitionPerDay as u32 ) , AttributeId :: ActivePresetHandle => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActivePresetHandle" , AttributeId :: ActivePresetHandle as u32 ) , AttributeId :: ActiveScheduleHandle => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveScheduleHandle" , AttributeId :: ActiveScheduleHandle as u32 ) , AttributeId :: Presets => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Presets" , AttributeId :: Presets as u32 ) , AttributeId :: Schedules => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Schedules" , AttributeId :: Schedules as u32 ) , AttributeId :: SetpointHoldExpiryTimestamp => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointHoldExpiryTimestamp" , AttributeId :: SetpointHoldExpiryTimestamp as u32 ) , AttributeId :: MaxThermostatSuggestions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxThermostatSuggestions" , AttributeId :: MaxThermostatSuggestions as u32 ) , AttributeId :: ThermostatSuggestions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThermostatSuggestions" , AttributeId :: ThermostatSuggestions as u32 ) , AttributeId :: CurrentThermostatSuggestion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentThermostatSuggestion" , AttributeId :: CurrentThermostatSuggestion as u32 ) , AttributeId :: ThermostatSuggestionNotFollowingReason => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThermostatSuggestionNotFollowingReason" , AttributeId :: ThermostatSuggestionNotFollowingReason as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    SetpointRaiseLower = 0 , SetWeeklySchedule = 1 , GetWeeklySchedule = 2 , ClearWeeklySchedule = 3 , SetActiveScheduleRequest = 5 , SetActivePresetRequest = 6 , AddThermostatSuggestion = 7 , RemoveThermostatSuggestion = 8 , AtomicRequest = 254 
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
            CommandId :: SetpointRaiseLower => write ! (f , "{}(0x{:02x})" , "SetpointRaiseLower" , CommandId :: SetpointRaiseLower as u32 ) ? , CommandId :: SetWeeklySchedule => write ! (f , "{}(0x{:02x})" , "SetWeeklySchedule" , CommandId :: SetWeeklySchedule as u32 ) ? , CommandId :: GetWeeklySchedule => write ! (f , "{}(0x{:02x})" , "GetWeeklySchedule" , CommandId :: GetWeeklySchedule as u32 ) ? , CommandId :: ClearWeeklySchedule => write ! (f , "{}(0x{:02x})" , "ClearWeeklySchedule" , CommandId :: ClearWeeklySchedule as u32 ) ? , CommandId :: SetActiveScheduleRequest => write ! (f , "{}(0x{:02x})" , "SetActiveScheduleRequest" , CommandId :: SetActiveScheduleRequest as u32 ) ? , CommandId :: SetActivePresetRequest => write ! (f , "{}(0x{:02x})" , "SetActivePresetRequest" , CommandId :: SetActivePresetRequest as u32 ) ? , CommandId :: AddThermostatSuggestion => write ! (f , "{}(0x{:02x})" , "AddThermostatSuggestion" , CommandId :: AddThermostatSuggestion as u32 ) ? , CommandId :: RemoveThermostatSuggestion => write ! (f , "{}(0x{:02x})" , "RemoveThermostatSuggestion" , CommandId :: RemoveThermostatSuggestion as u32 ) ? , CommandId :: AtomicRequest => write ! (f , "{}(0x{:02x})" , "AtomicRequest" , CommandId :: AtomicRequest as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: SetpointRaiseLower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointRaiseLower" , CommandId :: SetpointRaiseLower as u32 ) , CommandId :: SetWeeklySchedule => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetWeeklySchedule" , CommandId :: SetWeeklySchedule as u32 ) , CommandId :: GetWeeklySchedule => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GetWeeklySchedule" , CommandId :: GetWeeklySchedule as u32 ) , CommandId :: ClearWeeklySchedule => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClearWeeklySchedule" , CommandId :: ClearWeeklySchedule as u32 ) , CommandId :: SetActiveScheduleRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetActiveScheduleRequest" , CommandId :: SetActiveScheduleRequest as u32 ) , CommandId :: SetActivePresetRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetActivePresetRequest" , CommandId :: SetActivePresetRequest as u32 ) , CommandId :: AddThermostatSuggestion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddThermostatSuggestion" , CommandId :: AddThermostatSuggestion as u32 ) , CommandId :: RemoveThermostatSuggestion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemoveThermostatSuggestion" , CommandId :: RemoveThermostatSuggestion as u32 ) , CommandId :: AtomicRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AtomicRequest" , CommandId :: AtomicRequest as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    GetWeeklyScheduleResponse = 0 , AddThermostatSuggestionResponse = 2 , AtomicResponse = 253 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: GetWeeklyScheduleResponse => write ! (f , "{}(0x{:02x})" , "GetWeeklyScheduleResponse" , CommandResponseId :: GetWeeklyScheduleResponse as u32 ) ? , CommandResponseId :: AddThermostatSuggestionResponse => write ! (f , "{}(0x{:02x})" , "AddThermostatSuggestionResponse" , CommandResponseId :: AddThermostatSuggestionResponse as u32 ) ? , CommandResponseId :: AtomicResponse => write ! (f , "{}(0x{:02x})" , "AtomicResponse" , CommandResponseId :: AtomicResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: GetWeeklyScheduleResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GetWeeklyScheduleResponse" , CommandResponseId :: GetWeeklyScheduleResponse as u32 ) , CommandResponseId :: AddThermostatSuggestionResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddThermostatSuggestionResponse" , CommandResponseId :: AddThermostatSuggestionResponse as u32 ) , CommandResponseId :: AtomicResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AtomicResponse" , CommandResponseId :: AtomicResponse as u32 ) , 
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
    SystemModeChange = 0 , LocalTemperatureChange = 1 , OccupancyChange = 2 , SetpointChange = 3 , RunningStateChange = 4 , RunningModeChange = 5 , ActiveScheduleChange = 6 , ActivePresetChange = 7 
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
            EventId :: SystemModeChange => write ! (f , "{}(0x{:02x})" , "SystemModeChange" , EventId :: SystemModeChange as u32 ) ? , EventId :: LocalTemperatureChange => write ! (f , "{}(0x{:02x})" , "LocalTemperatureChange" , EventId :: LocalTemperatureChange as u32 ) ? , EventId :: OccupancyChange => write ! (f , "{}(0x{:02x})" , "OccupancyChange" , EventId :: OccupancyChange as u32 ) ? , EventId :: SetpointChange => write ! (f , "{}(0x{:02x})" , "SetpointChange" , EventId :: SetpointChange as u32 ) ? , EventId :: RunningStateChange => write ! (f , "{}(0x{:02x})" , "RunningStateChange" , EventId :: RunningStateChange as u32 ) ? , EventId :: RunningModeChange => write ! (f , "{}(0x{:02x})" , "RunningModeChange" , EventId :: RunningModeChange as u32 ) ? , EventId :: ActiveScheduleChange => write ! (f , "{}(0x{:02x})" , "ActiveScheduleChange" , EventId :: ActiveScheduleChange as u32 ) ? , EventId :: ActivePresetChange => write ! (f , "{}(0x{:02x})" , "ActivePresetChange" , EventId :: ActivePresetChange as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: SystemModeChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SystemModeChange" , EventId :: SystemModeChange as u32 ) , EventId :: LocalTemperatureChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocalTemperatureChange" , EventId :: LocalTemperatureChange as u32 ) , EventId :: OccupancyChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OccupancyChange" , EventId :: OccupancyChange as u32 ) , EventId :: SetpointChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetpointChange" , EventId :: SetpointChange as u32 ) , EventId :: RunningStateChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RunningStateChange" , EventId :: RunningStateChange as u32 ) , EventId :: RunningModeChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RunningModeChange" , EventId :: RunningModeChange as u32 ) , EventId :: ActiveScheduleChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveScheduleChange" , EventId :: ActiveScheduleChange as u32 ) , EventId :: ActivePresetChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActivePresetChange" , EventId :: ActivePresetChange as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
use super :: globals :: * ;
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (513 , 11 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: LocalTemperature as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: OutdoorTemperature as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Occupancy as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AbsMinHeatSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AbsMaxHeatSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AbsMinCoolSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AbsMaxCoolSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: PICoolingDemand as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: PIHeatingDemand as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: HVACSystemTypeConfiguration as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: LocalTemperatureCalibration as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OccupiedCoolingSetpoint as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OccupiedHeatingSetpoint as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: UnoccupiedCoolingSetpoint as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: UnoccupiedHeatingSetpoint as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinHeatSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxHeatSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinCoolSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxCoolSetpointLimit as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinSetpointDeadBand as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RemoteSensing as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ControlSequenceOfOperation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: SystemMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ThermostatRunningMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: StartOfWeek as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfWeeklyTransitions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfDailyTransitions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TemperatureSetpointHold as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TemperatureSetpointHoldDuration as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ThermostatProgrammingOperationMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ThermostatRunningState as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SetpointChangeSource as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SetpointChangeAmount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SetpointChangeSourceTimestamp as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OccupiedSetback as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OccupiedSetbackMin as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OccupiedSetbackMax as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: UnoccupiedSetback as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: UnoccupiedSetbackMin as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: UnoccupiedSetbackMax as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: EmergencyHeatDelta as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACType as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACCapacity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACRefrigerantType as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACCompressorType as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACErrorCode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACLouverPosition as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACCoilTemperature as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ACCapacityformat as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: PresetTypes as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: ScheduleTypes as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfPresets as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfSchedules as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfScheduleTransitions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfScheduleTransitionPerDay as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActivePresetHandle as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveScheduleHandle as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Presets as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: Schedules as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: SetpointHoldExpiryTimestamp as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxThermostatSuggestions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ThermostatSuggestions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentThermostatSuggestion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ThermostatSuggestionNotFollowingReason as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: SetpointRaiseLower as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: SetWeeklySchedule as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: GetWeeklySchedule as _ , Some (CommandResponseId :: GetWeeklyScheduleResponse as _ ) , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: ClearWeeklySchedule as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: SetActiveScheduleRequest as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: SetActivePresetRequest as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: AddThermostatSuggestion as _ , Some (CommandResponseId :: AddThermostatSuggestionResponse as _ ) , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: RemoveThermostatSuggestion as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: AtomicRequest as _ , Some (CommandResponseId :: AtomicResponse as _ ) , crate :: dm :: Access :: WM , ) , ] , & [crate :: dm :: Event :: new (EventId :: SystemModeChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: LocalTemperatureChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: OccupancyChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: SetpointChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: RunningStateChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: RunningModeChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: ActiveScheduleChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: ActivePresetChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn local_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > ;
    fn outdoor_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupancy (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OccupancyBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn abs_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn abs_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn abs_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn abs_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn pi_cooling_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn pi_heating_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn hvac_system_type_configuration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < HVACSystemTypeBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn local_temperature_calibration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_setpoint_dead_band (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn remote_sensing (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RemoteSensingBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn control_sequence_of_operation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ControlSequenceOfOperationEnum , crate :: error :: Error > ;
    fn system_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < SystemModeEnum , crate :: error :: Error > ;
    fn thermostat_running_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThermostatRunningModeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn start_of_week (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < StartOfWeekEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_weekly_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_daily_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn temperature_setpoint_hold (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TemperatureSetpointHoldEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ProgrammingOperationModeBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn thermostat_running_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RelayStateBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn setpoint_change_source (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < SetpointChangeSourceEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn setpoint_change_amount (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn setpoint_change_source_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn occupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn unoccupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn unoccupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn unoccupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn emergency_heat_delta (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACTypeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_refrigerant_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACRefrigerantTypeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_compressor_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACCompressorTypeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_error_code (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACErrorCodeBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_louver_position (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACLouverPositionEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_coil_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn ac_capacityformat (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACCapacityFormatEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn preset_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetTypeStructArrayBuilder < P > , PresetTypeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn schedule_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleTypeStructArrayBuilder < P > , ScheduleTypeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_presets (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_schedules (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_schedule_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn number_of_schedule_transition_per_day (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_preset_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_schedule_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn presets < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetStructArrayBuilder < P > , PresetStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn schedules < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleStructArrayBuilder < P > , ScheduleStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn setpoint_hold_expiry_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_thermostat_suggestions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn thermostat_suggestions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ThermostatSuggestionStructArrayBuilder < P > , ThermostatSuggestionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ThermostatSuggestionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn thermostat_suggestion_not_following_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_hvac_system_type_configuration (& self , ctx : impl crate :: dm :: WriteContext , value : HVACSystemTypeBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_local_temperature_calibration (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_occupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_min_setpoint_dead_band (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_remote_sensing (& self , ctx : impl crate :: dm :: WriteContext , value : RemoteSensingBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_control_sequence_of_operation (& self , ctx : impl crate :: dm :: WriteContext , value : ControlSequenceOfOperationEnum ) -> Result < () , crate :: error :: Error > ;
    fn set_system_mode (& self , ctx : impl crate :: dm :: WriteContext , value : SystemModeEnum ) -> Result < () , crate :: error :: Error > ;
    fn set_temperature_setpoint_hold (& self , ctx : impl crate :: dm :: WriteContext , value : TemperatureSetpointHoldEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: WriteContext , value : ProgrammingOperationModeBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_occupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_unoccupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_emergency_heat_delta (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACTypeEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_capacity (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_refrigerant_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACRefrigerantTypeEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_compressor_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACCompressorTypeEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_error_code (& self , ctx : impl crate :: dm :: WriteContext , value : ACErrorCodeBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_louver_position (& self , ctx : impl crate :: dm :: WriteContext , value : ACLouverPositionEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_ac_capacityformat (& self , ctx : impl crate :: dm :: WriteContext , value : ACCapacityFormatEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_presets (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , PresetStruct < '_ > > , PresetStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_schedules (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ScheduleStruct < '_ > > , ScheduleStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_setpoint_raise_lower (& self , ctx : impl crate :: dm :: InvokeContext , request : SetpointRaiseLowerRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , request : SetWeeklyScheduleRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_get_weekly_schedule < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetWeeklyScheduleRequest < '_ > , response : GetWeeklyScheduleResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_clear_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_active_schedule_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActiveScheduleRequestRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_active_preset_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActivePresetRequestRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_add_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddThermostatSuggestionRequest < '_ > , response : AddThermostatSuggestionResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_remove_thermostat_suggestion (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveThermostatSuggestionRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_atomic_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AtomicRequestRequest < '_ > , response : AtomicResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn local_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        T :: local_temperature (self , ctx ) 
    }
    fn outdoor_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        T :: outdoor_temperature (self , ctx ) 
    }
    fn occupancy (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OccupancyBitmap , crate :: error :: Error > {
        T :: occupancy (self , ctx ) 
    }
    fn abs_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: abs_min_heat_setpoint_limit (self , ctx ) 
    }
    fn abs_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: abs_max_heat_setpoint_limit (self , ctx ) 
    }
    fn abs_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: abs_min_cool_setpoint_limit (self , ctx ) 
    }
    fn abs_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: abs_max_cool_setpoint_limit (self , ctx ) 
    }
    fn pi_cooling_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: pi_cooling_demand (self , ctx ) 
    }
    fn pi_heating_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: pi_heating_demand (self , ctx ) 
    }
    fn hvac_system_type_configuration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < HVACSystemTypeBitmap , crate :: error :: Error > {
        T :: hvac_system_type_configuration (self , ctx ) 
    }
    fn local_temperature_calibration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i8 , crate :: error :: Error > {
        T :: local_temperature_calibration (self , ctx ) 
    }
    fn occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: occupied_cooling_setpoint (self , ctx ) 
    }
    fn occupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: occupied_heating_setpoint (self , ctx ) 
    }
    fn unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: unoccupied_cooling_setpoint (self , ctx ) 
    }
    fn unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: unoccupied_heating_setpoint (self , ctx ) 
    }
    fn min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: min_heat_setpoint_limit (self , ctx ) 
    }
    fn max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: max_heat_setpoint_limit (self , ctx ) 
    }
    fn min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: min_cool_setpoint_limit (self , ctx ) 
    }
    fn max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i16 , crate :: error :: Error > {
        T :: max_cool_setpoint_limit (self , ctx ) 
    }
    fn min_setpoint_dead_band (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < i8 , crate :: error :: Error > {
        T :: min_setpoint_dead_band (self , ctx ) 
    }
    fn remote_sensing (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RemoteSensingBitmap , crate :: error :: Error > {
        T :: remote_sensing (self , ctx ) 
    }
    fn control_sequence_of_operation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ControlSequenceOfOperationEnum , crate :: error :: Error > {
        T :: control_sequence_of_operation (self , ctx ) 
    }
    fn system_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < SystemModeEnum , crate :: error :: Error > {
        T :: system_mode (self , ctx ) 
    }
    fn thermostat_running_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThermostatRunningModeEnum , crate :: error :: Error > {
        T :: thermostat_running_mode (self , ctx ) 
    }
    fn start_of_week (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < StartOfWeekEnum , crate :: error :: Error > {
        T :: start_of_week (self , ctx ) 
    }
    fn number_of_weekly_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_weekly_transitions (self , ctx ) 
    }
    fn number_of_daily_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_daily_transitions (self , ctx ) 
    }
    fn temperature_setpoint_hold (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TemperatureSetpointHoldEnum , crate :: error :: Error > {
        T :: temperature_setpoint_hold (self , ctx ) 
    }
    fn temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: temperature_setpoint_hold_duration (self , ctx ) 
    }
    fn thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ProgrammingOperationModeBitmap , crate :: error :: Error > {
        T :: thermostat_programming_operation_mode (self , ctx ) 
    }
    fn thermostat_running_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RelayStateBitmap , crate :: error :: Error > {
        T :: thermostat_running_state (self , ctx ) 
    }
    fn setpoint_change_source (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < SetpointChangeSourceEnum , crate :: error :: Error > {
        T :: setpoint_change_source (self , ctx ) 
    }
    fn setpoint_change_amount (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        T :: setpoint_change_amount (self , ctx ) 
    }
    fn setpoint_change_source_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: setpoint_change_source_timestamp (self , ctx ) 
    }
    fn occupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: occupied_setback (self , ctx ) 
    }
    fn occupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: occupied_setback_min (self , ctx ) 
    }
    fn occupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: occupied_setback_max (self , ctx ) 
    }
    fn unoccupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: unoccupied_setback (self , ctx ) 
    }
    fn unoccupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: unoccupied_setback_min (self , ctx ) 
    }
    fn unoccupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: unoccupied_setback_max (self , ctx ) 
    }
    fn emergency_heat_delta (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: emergency_heat_delta (self , ctx ) 
    }
    fn ac_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACTypeEnum , crate :: error :: Error > {
        T :: ac_type (self , ctx ) 
    }
    fn ac_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: ac_capacity (self , ctx ) 
    }
    fn ac_refrigerant_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACRefrigerantTypeEnum , crate :: error :: Error > {
        T :: ac_refrigerant_type (self , ctx ) 
    }
    fn ac_compressor_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACCompressorTypeEnum , crate :: error :: Error > {
        T :: ac_compressor_type (self , ctx ) 
    }
    fn ac_error_code (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACErrorCodeBitmap , crate :: error :: Error > {
        T :: ac_error_code (self , ctx ) 
    }
    fn ac_louver_position (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACLouverPositionEnum , crate :: error :: Error > {
        T :: ac_louver_position (self , ctx ) 
    }
    fn ac_coil_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > {
        T :: ac_coil_temperature (self , ctx ) 
    }
    fn ac_capacityformat (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ACCapacityFormatEnum , crate :: error :: Error > {
        T :: ac_capacityformat (self , ctx ) 
    }
    fn preset_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetTypeStructArrayBuilder < P > , PresetTypeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: preset_types (self , ctx , builder ) 
    }
    fn schedule_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleTypeStructArrayBuilder < P > , ScheduleTypeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: schedule_types (self , ctx , builder ) 
    }
    fn number_of_presets (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_presets (self , ctx ) 
    }
    fn number_of_schedules (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_schedules (self , ctx ) 
    }
    fn number_of_schedule_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_schedule_transitions (self , ctx ) 
    }
    fn number_of_schedule_transition_per_day (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: number_of_schedule_transition_per_day (self , ctx ) 
    }
    fn active_preset_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: active_preset_handle (self , ctx , builder ) 
    }
    fn active_schedule_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: active_schedule_handle (self , ctx , builder ) 
    }
    fn presets < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetStructArrayBuilder < P > , PresetStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: presets (self , ctx , builder ) 
    }
    fn schedules < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleStructArrayBuilder < P > , ScheduleStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: schedules (self , ctx , builder ) 
    }
    fn setpoint_hold_expiry_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: setpoint_hold_expiry_timestamp (self , ctx ) 
    }
    fn max_thermostat_suggestions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: max_thermostat_suggestions (self , ctx ) 
    }
    fn thermostat_suggestions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ThermostatSuggestionStructArrayBuilder < P > , ThermostatSuggestionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: thermostat_suggestions (self , ctx , builder ) 
    }
    fn current_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ThermostatSuggestionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: current_thermostat_suggestion (self , ctx , builder ) 
    }
    fn thermostat_suggestion_not_following_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error > {
        T :: thermostat_suggestion_not_following_reason (self , ctx ) 
    }
    fn set_hvac_system_type_configuration (& self , ctx : impl crate :: dm :: WriteContext , value : HVACSystemTypeBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_hvac_system_type_configuration (self , ctx , value ) 
    }
    fn set_local_temperature_calibration (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> Result < () , crate :: error :: Error > {
        T :: set_local_temperature_calibration (self , ctx , value ) 
    }
    fn set_occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_occupied_cooling_setpoint (self , ctx , value ) 
    }
    fn set_occupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_occupied_heating_setpoint (self , ctx , value ) 
    }
    fn set_unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_unoccupied_cooling_setpoint (self , ctx , value ) 
    }
    fn set_unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_unoccupied_heating_setpoint (self , ctx , value ) 
    }
    fn set_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_min_heat_setpoint_limit (self , ctx , value ) 
    }
    fn set_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_max_heat_setpoint_limit (self , ctx , value ) 
    }
    fn set_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_min_cool_setpoint_limit (self , ctx , value ) 
    }
    fn set_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> Result < () , crate :: error :: Error > {
        T :: set_max_cool_setpoint_limit (self , ctx , value ) 
    }
    fn set_min_setpoint_dead_band (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> Result < () , crate :: error :: Error > {
        T :: set_min_setpoint_dead_band (self , ctx , value ) 
    }
    fn set_remote_sensing (& self , ctx : impl crate :: dm :: WriteContext , value : RemoteSensingBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_remote_sensing (self , ctx , value ) 
    }
    fn set_control_sequence_of_operation (& self , ctx : impl crate :: dm :: WriteContext , value : ControlSequenceOfOperationEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_control_sequence_of_operation (self , ctx , value ) 
    }
    fn set_system_mode (& self , ctx : impl crate :: dm :: WriteContext , value : SystemModeEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_system_mode (self , ctx , value ) 
    }
    fn set_temperature_setpoint_hold (& self , ctx : impl crate :: dm :: WriteContext , value : TemperatureSetpointHoldEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_temperature_setpoint_hold (self , ctx , value ) 
    }
    fn set_temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        T :: set_temperature_setpoint_hold_duration (self , ctx , value ) 
    }
    fn set_thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: WriteContext , value : ProgrammingOperationModeBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_thermostat_programming_operation_mode (self , ctx , value ) 
    }
    fn set_occupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_occupied_setback (self , ctx , value ) 
    }
    fn set_unoccupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_unoccupied_setback (self , ctx , value ) 
    }
    fn set_emergency_heat_delta (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        T :: set_emergency_heat_delta (self , ctx , value ) 
    }
    fn set_ac_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACTypeEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_type (self , ctx , value ) 
    }
    fn set_ac_capacity (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_capacity (self , ctx , value ) 
    }
    fn set_ac_refrigerant_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACRefrigerantTypeEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_refrigerant_type (self , ctx , value ) 
    }
    fn set_ac_compressor_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACCompressorTypeEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_compressor_type (self , ctx , value ) 
    }
    fn set_ac_error_code (& self , ctx : impl crate :: dm :: WriteContext , value : ACErrorCodeBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_error_code (self , ctx , value ) 
    }
    fn set_ac_louver_position (& self , ctx : impl crate :: dm :: WriteContext , value : ACLouverPositionEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_louver_position (self , ctx , value ) 
    }
    fn set_ac_capacityformat (& self , ctx : impl crate :: dm :: WriteContext , value : ACCapacityFormatEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_ac_capacityformat (self , ctx , value ) 
    }
    fn set_presets (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , PresetStruct < '_ > > , PresetStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        T :: set_presets (self , ctx , value ) 
    }
    fn set_schedules (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ScheduleStruct < '_ > > , ScheduleStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        T :: set_schedules (self , ctx , value ) 
    }
    fn handle_setpoint_raise_lower (& self , ctx : impl crate :: dm :: InvokeContext , request : SetpointRaiseLowerRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_setpoint_raise_lower (self , ctx , request ) 
    }
    fn handle_set_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , request : SetWeeklyScheduleRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_weekly_schedule (self , ctx , request ) 
    }
    fn handle_get_weekly_schedule < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetWeeklyScheduleRequest < '_ > , response : GetWeeklyScheduleResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_get_weekly_schedule (self , ctx , request , response ) 
    }
    fn handle_clear_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , ) -> Result < () , crate :: error :: Error > {
        T :: handle_clear_weekly_schedule (self , ctx ) 
    }
    fn handle_set_active_schedule_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActiveScheduleRequestRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_active_schedule_request (self , ctx , request ) 
    }
    fn handle_set_active_preset_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActivePresetRequestRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_active_preset_request (self , ctx , request ) 
    }
    fn handle_add_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddThermostatSuggestionRequest < '_ > , response : AddThermostatSuggestionResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_add_thermostat_suggestion (self , ctx , request , response ) 
    }
    fn handle_remove_thermostat_suggestion (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveThermostatSuggestionRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_remove_thermostat_suggestion (self , ctx , request ) 
    }
    fn handle_atomic_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AtomicRequestRequest < '_ > , response : AtomicResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_atomic_request (self , ctx , request , response ) 
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
                    AttributeId :: LocalTemperature => {
                        let attr_read_result = self . 0. local_temperature (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OutdoorTemperature => {
                        let attr_read_result = self . 0. outdoor_temperature (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OutdoorTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Occupancy => {
                        let attr_read_result = self . 0. occupancy (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Occupancy , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMinHeatSetpointLimit => {
                        let attr_read_result = self . 0. abs_min_heat_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMinHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMaxHeatSetpointLimit => {
                        let attr_read_result = self . 0. abs_max_heat_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMaxHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMinCoolSetpointLimit => {
                        let attr_read_result = self . 0. abs_min_cool_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMinCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMaxCoolSetpointLimit => {
                        let attr_read_result = self . 0. abs_max_cool_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMaxCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PICoolingDemand => {
                        let attr_read_result = self . 0. pi_cooling_demand (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PICoolingDemand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PIHeatingDemand => {
                        let attr_read_result = self . 0. pi_heating_demand (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PIHeatingDemand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HVACSystemTypeConfiguration => {
                        let attr_read_result = self . 0. hvac_system_type_configuration (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HVACSystemTypeConfiguration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalTemperatureCalibration => {
                        let attr_read_result = self . 0. local_temperature_calibration (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperatureCalibration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedCoolingSetpoint => {
                        let attr_read_result = self . 0. occupied_cooling_setpoint (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedCoolingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedHeatingSetpoint => {
                        let attr_read_result = self . 0. occupied_heating_setpoint (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedHeatingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedCoolingSetpoint => {
                        let attr_read_result = self . 0. unoccupied_cooling_setpoint (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedCoolingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedHeatingSetpoint => {
                        let attr_read_result = self . 0. unoccupied_heating_setpoint (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedHeatingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinHeatSetpointLimit => {
                        let attr_read_result = self . 0. min_heat_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxHeatSetpointLimit => {
                        let attr_read_result = self . 0. max_heat_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinCoolSetpointLimit => {
                        let attr_read_result = self . 0. min_cool_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxCoolSetpointLimit => {
                        let attr_read_result = self . 0. max_cool_setpoint_limit (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinSetpointDeadBand => {
                        let attr_read_result = self . 0. min_setpoint_dead_band (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinSetpointDeadBand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemoteSensing => {
                        let attr_read_result = self . 0. remote_sensing (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemoteSensing , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ControlSequenceOfOperation => {
                        let attr_read_result = self . 0. control_sequence_of_operation (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ControlSequenceOfOperation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SystemMode => {
                        let attr_read_result = self . 0. system_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SystemMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatRunningMode => {
                        let attr_read_result = self . 0. thermostat_running_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatRunningMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartOfWeek => {
                        let attr_read_result = self . 0. start_of_week (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartOfWeek , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfWeeklyTransitions => {
                        let attr_read_result = self . 0. number_of_weekly_transitions (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfWeeklyTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfDailyTransitions => {
                        let attr_read_result = self . 0. number_of_daily_transitions (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfDailyTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TemperatureSetpointHold => {
                        let attr_read_result = self . 0. temperature_setpoint_hold (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHold , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TemperatureSetpointHoldDuration => {
                        let attr_read_result = self . 0. temperature_setpoint_hold_duration (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHoldDuration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatProgrammingOperationMode => {
                        let attr_read_result = self . 0. thermostat_programming_operation_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatProgrammingOperationMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatRunningState => {
                        let attr_read_result = self . 0. thermostat_running_state (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatRunningState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeSource => {
                        let attr_read_result = self . 0. setpoint_change_source (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeSource , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeAmount => {
                        let attr_read_result = self . 0. setpoint_change_amount (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeAmount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeSourceTimestamp => {
                        let attr_read_result = self . 0. setpoint_change_source_timestamp (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeSourceTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetback => {
                        let attr_read_result = self . 0. occupied_setback (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetback , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetbackMin => {
                        let attr_read_result = self . 0. occupied_setback_min (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetbackMin , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetbackMax => {
                        let attr_read_result = self . 0. occupied_setback_max (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetbackMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetback => {
                        let attr_read_result = self . 0. unoccupied_setback (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetback , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetbackMin => {
                        let attr_read_result = self . 0. unoccupied_setback_min (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetbackMin , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetbackMax => {
                        let attr_read_result = self . 0. unoccupied_setback_max (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetbackMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EmergencyHeatDelta => {
                        let attr_read_result = self . 0. emergency_heat_delta (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EmergencyHeatDelta , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACType => {
                        let attr_read_result = self . 0. ac_type (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCapacity => {
                        let attr_read_result = self . 0. ac_capacity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACRefrigerantType => {
                        let attr_read_result = self . 0. ac_refrigerant_type (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACRefrigerantType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCompressorType => {
                        let attr_read_result = self . 0. ac_compressor_type (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCompressorType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACErrorCode => {
                        let attr_read_result = self . 0. ac_error_code (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACErrorCode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACLouverPosition => {
                        let attr_read_result = self . 0. ac_louver_position (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACLouverPosition , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCoilTemperature => {
                        let attr_read_result = self . 0. ac_coil_temperature (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCoilTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCapacityformat => {
                        let attr_read_result = self . 0. ac_capacityformat (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacityformat , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PresetTypes => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. preset_types (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ScheduleTypes => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. schedule_types (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NumberOfPresets => {
                        let attr_read_result = self . 0. number_of_presets (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPresets , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfSchedules => {
                        let attr_read_result = self . 0. number_of_schedules (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfSchedules , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfScheduleTransitions => {
                        let attr_read_result = self . 0. number_of_schedule_transitions (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfScheduleTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfScheduleTransitionPerDay => {
                        let attr_read_result = self . 0. number_of_schedule_transition_per_day (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfScheduleTransitionPerDay , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActivePresetHandle => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_preset_handle (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveScheduleHandle => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_schedule_handle (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Presets => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. presets (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Schedules => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. schedules (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SetpointHoldExpiryTimestamp => {
                        let attr_read_result = self . 0. setpoint_hold_expiry_timestamp (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointHoldExpiryTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxThermostatSuggestions => {
                        let attr_read_result = self . 0. max_thermostat_suggestions (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxThermostatSuggestions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatSuggestions => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. thermostat_suggestions (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentThermostatSuggestion => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_thermostat_suggestion (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ThermostatSuggestionNotFollowingReason => {
                        let attr_read_result = self . 0. thermostat_suggestion_not_following_reason (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestionNotFollowingReason , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: HVACSystemTypeConfiguration => {
                let attr_data : HVACSystemTypeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_hvac_system_type_configuration (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HVACSystemTypeConfiguration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalTemperatureCalibration => {
                let attr_data : i8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_temperature_calibration (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperatureCalibration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedCoolingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_cooling_setpoint (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedCoolingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedHeatingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_heating_setpoint (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedHeatingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedCoolingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_cooling_setpoint (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedCoolingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedHeatingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_heating_setpoint (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedHeatingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinHeatSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_heat_setpoint_limit (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinHeatSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MaxHeatSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_max_heat_setpoint_limit (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxHeatSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinCoolSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_cool_setpoint_limit (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinCoolSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MaxCoolSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_max_cool_setpoint_limit (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxCoolSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinSetpointDeadBand => {
                let attr_data : i8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_setpoint_dead_band (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinSetpointDeadBand , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: RemoteSensing => {
                let attr_data : RemoteSensingBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_remote_sensing (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemoteSensing , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ControlSequenceOfOperation => {
                let attr_data : ControlSequenceOfOperationEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_control_sequence_of_operation (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ControlSequenceOfOperation , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SystemMode => {
                let attr_data : SystemModeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_system_mode (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SystemMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: TemperatureSetpointHold => {
                let attr_data : TemperatureSetpointHoldEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_temperature_setpoint_hold (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHold , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: TemperatureSetpointHoldDuration => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_temperature_setpoint_hold_duration (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHoldDuration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ThermostatProgrammingOperationMode => {
                let attr_data : ProgrammingOperationModeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_thermostat_programming_operation_mode (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatProgrammingOperationMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedSetback => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_setback (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetback , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedSetback => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_setback (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetback , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: EmergencyHeatDelta => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_emergency_heat_delta (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EmergencyHeatDelta , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACType => {
                let attr_data : ACTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_type (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCapacity => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_capacity (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacity , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACRefrigerantType => {
                let attr_data : ACRefrigerantTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_refrigerant_type (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACRefrigerantType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCompressorType => {
                let attr_data : ACCompressorTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_compressor_type (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCompressorType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACErrorCode => {
                let attr_data : ACErrorCodeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_error_code (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACErrorCode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACLouverPosition => {
                let attr_data : ACLouverPositionEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_louver_position (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACLouverPosition , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCapacityformat => {
                let attr_data : ACCapacityFormatEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_capacityformat (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacityformat , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Presets => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_presets (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Schedules => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_schedules (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: SetpointRaiseLower => {
                let cmd_data : SetpointRaiseLowerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_setpoint_raise_lower (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetpointRaiseLower ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetWeeklySchedule => {
                let cmd_data : SetWeeklyScheduleRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_weekly_schedule (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetWeeklySchedule ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GetWeeklySchedule => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (0u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_get_weekly_schedule (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ClearWeeklySchedule => {
                let cmd_invoke_result = self . 0. handle_clear_weekly_schedule (& ctx ) ;
                crate :: dm :: log_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ClearWeeklySchedule ) ) ) , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetActiveScheduleRequest => {
                let cmd_data : SetActiveScheduleRequestRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_active_schedule_request (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetActiveScheduleRequest ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetActivePresetRequest => {
                let cmd_data : SetActivePresetRequestRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_active_preset_request (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetActivePresetRequest ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: AddThermostatSuggestion => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_thermostat_suggestion (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveThermostatSuggestion => {
                let cmd_data : RemoveThermostatSuggestionRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_remove_thermostat_suggestion (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveThermostatSuggestion ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: AtomicRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (253u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_atomic_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 513u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Thermostat" , 513u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Thermostat" , 513u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `Thermostat` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct ThermostatCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > ThermostatCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn setpoint_raise_lower (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetpointRaiseLowerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 0 ) ? . data_builder () 
    }
    pub fn set_weekly_schedule (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetWeeklyScheduleRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 1 ) ? . data_builder () 
    }
    pub fn get_weekly_schedule (self , endpoint : crate :: dm :: EndptId , ) -> Result < GetWeeklyScheduleRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 2 ) ? . data_builder () 
    }
    pub fn clear_weekly_schedule (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        let array = self . array . push () ? . path (endpoint , 513 , 3 ) ? . data (| w | {
            w . start_struct (& crate :: tlv :: TLVTag :: Context (crate :: im :: CmdDataTag :: Data as u8 , ) ) ? ;
            w . end_container () 
        }
        ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn set_active_schedule_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetActiveScheduleRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 5 ) ? . data_builder () 
    }
    pub fn set_active_preset_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetActivePresetRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 6 ) ? . data_builder () 
    }
    pub fn add_thermostat_suggestion (self , endpoint : crate :: dm :: EndptId , ) -> Result < AddThermostatSuggestionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 7 ) ? . data_builder () 
    }
    pub fn remove_thermostat_suggestion (self , endpoint : crate :: dm :: EndptId , ) -> Result < RemoveThermostatSuggestionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 8 ) ? . data_builder () 
    }
    pub fn atomic_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < AtomicRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 513 , 254 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Thermostat` cluster's commands. `use` this trait to call `.thermostat_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`ThermostatCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThermostatCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_inv (self ) -> ThermostatCmdRequestsView < P > ;
    
}
impl < P > ThermostatCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_inv (self ) -> ThermostatCmdRequestsView < P > {
        ThermostatCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `Thermostat` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThermostatAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > ThermostatAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn local_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn outdoor_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupancy (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn abs_min_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn abs_max_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn abs_min_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn abs_max_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn pi_cooling_demand (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn pi_heating_demand (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn hvac_system_type_configuration (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_temperature_calibration (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_cooling_setpoint (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_heating_setpoint (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_cooling_setpoint (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_heating_setpoint (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (20 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (21 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (22 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (23 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (24 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_setpoint_dead_band (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (25 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn remote_sensing (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (26 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn control_sequence_of_operation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (27 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn system_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (28 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_running_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (30 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn start_of_week (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (32 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_weekly_transitions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (33 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_daily_transitions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (34 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn temperature_setpoint_hold (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (35 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_duration (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (36 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_programming_operation_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (37 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_running_state (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (41 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn setpoint_change_source (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (48 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn setpoint_change_amount (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (49 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn setpoint_change_source_timestamp (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (50 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_setback (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (52 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_setback_min (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (53 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_setback_max (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (54 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_setback (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (55 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_setback_min (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (56 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_setback_max (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (57 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn emergency_heat_delta (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (58 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_type (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (64 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_capacity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_refrigerant_type (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (66 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_compressor_type (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (67 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_error_code (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (68 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_louver_position (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (69 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_coil_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (70 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_capacityformat (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (71 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn preset_types (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (72 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn schedule_types (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (73 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_presets (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (74 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_schedules (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (75 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_schedule_transitions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (76 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_schedule_transition_per_day (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (77 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_preset_handle (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (78 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_schedule_handle (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (79 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn presets (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (80 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn schedules (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (81 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn setpoint_hold_expiry_timestamp (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (82 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_thermostat_suggestions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (83 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_suggestions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (84 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_thermostat_suggestion (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (85 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_suggestion_not_following_reason (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (86 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (513 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Thermostat` cluster's attribute reads. `use` this trait to call `.thermostat_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`ThermostatAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThermostatAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_read (self ) -> ThermostatAttrReadsView < P > ;
    
}
impl < P > ThermostatAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_read (self ) -> ThermostatAttrReadsView < P > {
        ThermostatAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `Thermostat` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThermostatAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > ThermostatAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hvac_system_type_configuration (self , endpoint : crate :: dm :: EndptId , value : HVACSystemTypeBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 9 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_temperature_calibration (self , endpoint : crate :: dm :: EndptId , value : i8 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 16 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_cooling_setpoint (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 17 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_heating_setpoint (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 18 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_cooling_setpoint (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 19 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_heating_setpoint (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 20 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 21 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_heat_setpoint_limit (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 22 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 23 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_cool_setpoint_limit (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 24 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_setpoint_dead_band (self , endpoint : crate :: dm :: EndptId , value : i8 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 25 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn remote_sensing (self , endpoint : crate :: dm :: EndptId , value : RemoteSensingBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 26 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn control_sequence_of_operation (self , endpoint : crate :: dm :: EndptId , value : ControlSequenceOfOperationEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 27 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn system_mode (self , endpoint : crate :: dm :: EndptId , value : SystemModeEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 28 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn temperature_setpoint_hold (self , endpoint : crate :: dm :: EndptId , value : TemperatureSetpointHoldEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 35 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_duration (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 36 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thermostat_programming_operation_mode (self , endpoint : crate :: dm :: EndptId , value : ProgrammingOperationModeBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 37 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn occupied_setback (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 52 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn unoccupied_setback (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 55 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn emergency_heat_delta (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 58 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_type (self , endpoint : crate :: dm :: EndptId , value : ACTypeEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 64 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_capacity (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 65 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_refrigerant_type (self , endpoint : crate :: dm :: EndptId , value : ACRefrigerantTypeEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 66 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_compressor_type (self , endpoint : crate :: dm :: EndptId , value : ACCompressorTypeEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 67 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_error_code (self , endpoint : crate :: dm :: EndptId , value : ACErrorCodeBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 68 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_louver_position (self , endpoint : crate :: dm :: EndptId , value : ACLouverPositionEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 69 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ac_capacityformat (self , endpoint : crate :: dm :: EndptId , value : ACCapacityFormatEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 513 , 71 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn presets (self , endpoint : crate :: dm :: EndptId , ) -> Result < PresetStructArrayBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 513 , 80 ) ? . data_builder () 
    }
    pub fn schedules (self , endpoint : crate :: dm :: EndptId , ) -> Result < ScheduleStructArrayBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 513 , 81 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Thermostat` cluster's attribute writes. `use` this trait to call `.thermostat_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`ThermostatAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThermostatAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_write (self ) -> ThermostatAttrWritesView < P > ;
    
}
impl < P > ThermostatAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thermostat_write (self ) -> ThermostatAttrWritesView < P > {
        ThermostatAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `Thermostat` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct ThermostatCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > ThermostatCmdResponsesView < 'a , 'r > {
    pub fn setpoint_raise_lower (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 0 ) 
    }
    pub fn set_weekly_schedule (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 1 ) 
    }
    pub fn get_weekly_schedule (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < GetWeeklyScheduleResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < GetWeeklyScheduleResponse < 'a >> (513 , 2 ) 
    }
    pub fn clear_weekly_schedule (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 3 ) 
    }
    pub fn set_active_schedule_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 5 ) 
    }
    pub fn set_active_preset_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 6 ) 
    }
    pub fn add_thermostat_suggestion (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AddThermostatSuggestionResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < AddThermostatSuggestionResponse < 'a >> (513 , 7 ) 
    }
    pub fn remove_thermostat_suggestion (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 8 ) 
    }
    pub fn atomic_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AtomicResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < AtomicResponse < 'a >> (513 , 254 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Thermostat`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.thermostat_inv_resp()` on an `InvokeResp`; the returned [`ThermostatCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait ThermostatCmdResponses < 'a > {
    fn thermostat_inv_resp (& self ) -> ThermostatCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThermostatCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn thermostat_inv_resp (& self ) -> ThermostatCmdResponsesView < 'a , '_ > {
        ThermostatCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `Thermostat` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThermostatAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > ThermostatAttrResponsesView < 'a , 'r > {
    pub fn local_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i16 > > (513 , 0 ) 
    }
    pub fn outdoor_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i16 > > (513 , 1 ) 
    }
    pub fn occupancy (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < OccupancyBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < OccupancyBitmap > (513 , 2 ) 
    }
    pub fn abs_min_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 3 ) 
    }
    pub fn abs_max_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 4 ) 
    }
    pub fn abs_min_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 5 ) 
    }
    pub fn abs_max_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 6 ) 
    }
    pub fn pi_cooling_demand (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 7 ) 
    }
    pub fn pi_heating_demand (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 8 ) 
    }
    pub fn hvac_system_type_configuration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < HVACSystemTypeBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < HVACSystemTypeBitmap > (513 , 9 ) 
    }
    pub fn local_temperature_calibration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i8 > (513 , 16 ) 
    }
    pub fn occupied_cooling_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 17 ) 
    }
    pub fn occupied_heating_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 18 ) 
    }
    pub fn unoccupied_cooling_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 19 ) 
    }
    pub fn unoccupied_heating_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 20 ) 
    }
    pub fn min_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 21 ) 
    }
    pub fn max_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 22 ) 
    }
    pub fn min_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 23 ) 
    }
    pub fn max_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i16 > (513 , 24 ) 
    }
    pub fn min_setpoint_dead_band (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < i8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < i8 > (513 , 25 ) 
    }
    pub fn remote_sensing (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RemoteSensingBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RemoteSensingBitmap > (513 , 26 ) 
    }
    pub fn control_sequence_of_operation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ControlSequenceOfOperationEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ControlSequenceOfOperationEnum > (513 , 27 ) 
    }
    pub fn system_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SystemModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < SystemModeEnum > (513 , 28 ) 
    }
    pub fn thermostat_running_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ThermostatRunningModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ThermostatRunningModeEnum > (513 , 30 ) 
    }
    pub fn start_of_week (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < StartOfWeekEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < StartOfWeekEnum > (513 , 32 ) 
    }
    pub fn number_of_weekly_transitions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 33 ) 
    }
    pub fn number_of_daily_transitions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 34 ) 
    }
    pub fn temperature_setpoint_hold (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < TemperatureSetpointHoldEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < TemperatureSetpointHoldEnum > (513 , 35 ) 
    }
    pub fn temperature_setpoint_hold_duration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (513 , 36 ) 
    }
    pub fn thermostat_programming_operation_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ProgrammingOperationModeBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ProgrammingOperationModeBitmap > (513 , 37 ) 
    }
    pub fn thermostat_running_state (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RelayStateBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RelayStateBitmap > (513 , 41 ) 
    }
    pub fn setpoint_change_source (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SetpointChangeSourceEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < SetpointChangeSourceEnum > (513 , 48 ) 
    }
    pub fn setpoint_change_amount (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i16 > > (513 , 49 ) 
    }
    pub fn setpoint_change_source_timestamp (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (513 , 50 ) 
    }
    pub fn occupied_setback (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 52 ) 
    }
    pub fn occupied_setback_min (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 53 ) 
    }
    pub fn occupied_setback_max (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 54 ) 
    }
    pub fn unoccupied_setback (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 55 ) 
    }
    pub fn unoccupied_setback_min (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 56 ) 
    }
    pub fn unoccupied_setback_max (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 57 ) 
    }
    pub fn emergency_heat_delta (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 58 ) 
    }
    pub fn ac_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACTypeEnum > (513 , 64 ) 
    }
    pub fn ac_capacity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (513 , 65 ) 
    }
    pub fn ac_refrigerant_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACRefrigerantTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACRefrigerantTypeEnum > (513 , 66 ) 
    }
    pub fn ac_compressor_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACCompressorTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACCompressorTypeEnum > (513 , 67 ) 
    }
    pub fn ac_error_code (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACErrorCodeBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACErrorCodeBitmap > (513 , 68 ) 
    }
    pub fn ac_louver_position (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACLouverPositionEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACLouverPositionEnum > (513 , 69 ) 
    }
    pub fn ac_coil_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i16 > > (513 , 70 ) 
    }
    pub fn ac_capacityformat (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ACCapacityFormatEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ACCapacityFormatEnum > (513 , 71 ) 
    }
    pub fn preset_types (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , PresetTypeStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , PresetTypeStruct < 'a > > > (513 , 72 ) 
    }
    pub fn schedule_types (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , ScheduleTypeStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , ScheduleTypeStruct < 'a > > > (513 , 73 ) 
    }
    pub fn number_of_presets (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 74 ) 
    }
    pub fn number_of_schedules (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 75 ) 
    }
    pub fn number_of_schedule_transitions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 76 ) 
    }
    pub fn number_of_schedule_transition_per_day (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (513 , 77 ) 
    }
    pub fn active_preset_handle (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > > (513 , 78 ) 
    }
    pub fn active_schedule_handle (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > > (513 , 79 ) 
    }
    pub fn presets (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , PresetStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , PresetStruct < 'a > > > (513 , 80 ) 
    }
    pub fn schedules (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , ScheduleStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , ScheduleStruct < 'a > > > (513 , 81 ) 
    }
    pub fn setpoint_hold_expiry_timestamp (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (513 , 82 ) 
    }
    pub fn max_thermostat_suggestions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (513 , 83 ) 
    }
    pub fn thermostat_suggestions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , ThermostatSuggestionStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , ThermostatSuggestionStruct < 'a > > > (513 , 84 ) 
    }
    pub fn current_thermostat_suggestion (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < ThermostatSuggestionStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < ThermostatSuggestionStruct < 'a > > > (513 , 85 ) 
    }
    pub fn thermostat_suggestion_not_following_reason (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > > (513 , 86 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (513 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (513 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (513 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (513 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (513 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Thermostat`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.thermostat_read_resp()` on a `ReportDataResp`; the returned [`ThermostatAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait ThermostatAttrResponses < 'a > {
    fn thermostat_read_resp (& self ) -> ThermostatAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThermostatAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn thermostat_read_resp (& self ) -> ThermostatAttrResponsesView < 'a , '_ > {
        ThermostatAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `Thermostat` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThermostatWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > ThermostatWriteResponsesView < 'a , 'r > {
    pub fn hvac_system_type_configuration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 9 ) 
    }
    pub fn local_temperature_calibration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 16 ) 
    }
    pub fn occupied_cooling_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 17 ) 
    }
    pub fn occupied_heating_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 18 ) 
    }
    pub fn unoccupied_cooling_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 19 ) 
    }
    pub fn unoccupied_heating_setpoint (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 20 ) 
    }
    pub fn min_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 21 ) 
    }
    pub fn max_heat_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 22 ) 
    }
    pub fn min_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 23 ) 
    }
    pub fn max_cool_setpoint_limit (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 24 ) 
    }
    pub fn min_setpoint_dead_band (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 25 ) 
    }
    pub fn remote_sensing (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 26 ) 
    }
    pub fn control_sequence_of_operation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 27 ) 
    }
    pub fn system_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 28 ) 
    }
    pub fn temperature_setpoint_hold (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 35 ) 
    }
    pub fn temperature_setpoint_hold_duration (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 36 ) 
    }
    pub fn thermostat_programming_operation_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 37 ) 
    }
    pub fn occupied_setback (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 52 ) 
    }
    pub fn unoccupied_setback (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 55 ) 
    }
    pub fn emergency_heat_delta (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 58 ) 
    }
    pub fn ac_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 64 ) 
    }
    pub fn ac_capacity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 65 ) 
    }
    pub fn ac_refrigerant_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 66 ) 
    }
    pub fn ac_compressor_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 67 ) 
    }
    pub fn ac_error_code (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 68 ) 
    }
    pub fn ac_louver_position (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 69 ) 
    }
    pub fn ac_capacityformat (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 71 ) 
    }
    pub fn presets (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 80 ) 
    }
    pub fn schedules (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (513 , 81 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Thermostat`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.thermostat_write_resp()` on a `WriteResp`; the returned [`ThermostatWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait ThermostatWriteResponses < 'a > {
    fn thermostat_write_resp (& self ) -> ThermostatWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThermostatWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn thermostat_write_resp (& self ) -> ThermostatWriteResponsesView < 'a , '_ > {
        ThermostatWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `GetWeeklyScheduleResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`GetWeeklyScheduleResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct GetWeeklyScheduleResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > GetWeeklyScheduleResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < GetWeeklyScheduleResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `AddThermostatSuggestionResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`AddThermostatSuggestionResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct AddThermostatSuggestionResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > AddThermostatSuggestionResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < AddThermostatSuggestionResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `AtomicResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`AtomicResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct AtomicResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > AtomicResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < AtomicResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `Thermostat` cluster's client operations. Returned by [`ThermostatClient::thermostat`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct ThermostatClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > ThermostatClientView < T > {
    pub fn setpoint_raise_lower < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetpointRaiseLowerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . setpoint_raise_lower (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_weekly_schedule < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetWeeklyScheduleRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . set_weekly_schedule (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn get_weekly_schedule < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < GetWeeklyScheduleResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (GetWeeklyScheduleRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . get_weekly_schedule (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (GetWeeklyScheduleResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn clear_weekly_schedule (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            msg . invoke_requests () ? . thermostat_inv () . clear_weekly_schedule (endpoint ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_active_schedule_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetActiveScheduleRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . set_active_schedule_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_active_preset_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetActivePresetRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . set_active_preset_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn add_thermostat_suggestion < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < AddThermostatSuggestionResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AddThermostatSuggestionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . add_thermostat_suggestion (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (AddThermostatSuggestionResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn remove_thermostat_suggestion < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (RemoveThermostatSuggestionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . remove_thermostat_suggestion (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn atomic_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < AtomicResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AtomicRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ThermostatCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . thermostat_inv () ;
            let req_builder = view . atomic_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (AtomicResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn local_temperature_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . local_temperature (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn outdoor_temperature_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . outdoor_temperature (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupancy_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < OccupancyBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupancy (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn abs_min_heat_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . abs_min_heat_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn abs_max_heat_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . abs_max_heat_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn abs_min_cool_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . abs_min_cool_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn abs_max_cool_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . abs_max_cool_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn pi_cooling_demand_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . pi_cooling_demand (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn pi_heating_demand_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . pi_heating_demand (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn hvac_system_type_configuration_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < HVACSystemTypeBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . hvac_system_type_configuration (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn local_temperature_calibration_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . local_temperature_calibration (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupied_cooling_setpoint_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupied_cooling_setpoint (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupied_heating_setpoint_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupied_heating_setpoint (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn unoccupied_cooling_setpoint_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . unoccupied_cooling_setpoint (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn unoccupied_heating_setpoint_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . unoccupied_heating_setpoint (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn min_heat_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . min_heat_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_heat_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . max_heat_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn min_cool_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . min_cool_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_cool_setpoint_limit_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . max_cool_setpoint_limit (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn min_setpoint_dead_band_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . min_setpoint_dead_band (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn remote_sensing_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RemoteSensingBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . remote_sensing (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn control_sequence_of_operation_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ControlSequenceOfOperationEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . control_sequence_of_operation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn system_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < SystemModeEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . system_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn thermostat_running_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ThermostatRunningModeEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . thermostat_running_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn start_of_week_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < StartOfWeekEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . start_of_week (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_weekly_transitions_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_weekly_transitions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_daily_transitions_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_daily_transitions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < TemperatureSetpointHoldEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . temperature_setpoint_hold (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_duration_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . temperature_setpoint_hold_duration (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn thermostat_programming_operation_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ProgrammingOperationModeBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . thermostat_programming_operation_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn thermostat_running_state_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RelayStateBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . thermostat_running_state (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn setpoint_change_source_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < SetpointChangeSourceEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . setpoint_change_source (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn setpoint_change_amount_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . setpoint_change_amount (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn setpoint_change_source_timestamp_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . setpoint_change_source_timestamp (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupied_setback_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupied_setback (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupied_setback_min_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupied_setback_min (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn occupied_setback_max_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . occupied_setback_max (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn unoccupied_setback_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . unoccupied_setback (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn unoccupied_setback_min_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . unoccupied_setback_min (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn unoccupied_setback_max_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . unoccupied_setback_max (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn emergency_heat_delta_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . emergency_heat_delta (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_type_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACTypeEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_type (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_capacity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_capacity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_refrigerant_type_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACRefrigerantTypeEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_refrigerant_type (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_compressor_type_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACCompressorTypeEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_compressor_type (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_error_code_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACErrorCodeBitmap , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_error_code (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_louver_position_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACLouverPositionEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_louver_position (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_coil_temperature_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_coil_temperature (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn ac_capacityformat_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ACCapacityFormatEnum , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . ac_capacityformat (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn preset_types_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , PresetTypeStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . preset_types (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn schedule_types_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , ScheduleTypeStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . schedule_types (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn number_of_presets_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_presets (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_schedules_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_schedules (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_schedule_transitions_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_schedule_transitions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_schedule_transition_per_day_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . number_of_schedule_transition_per_day (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn active_preset_handle_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . active_preset_handle (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn active_schedule_handle_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . active_schedule_handle (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn presets_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , PresetStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . presets (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn schedules_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , ScheduleStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . schedules (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn setpoint_hold_expiry_timestamp_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . setpoint_hold_expiry_timestamp (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_thermostat_suggestions_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . max_thermostat_suggestions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn thermostat_suggestions_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , ThermostatSuggestionStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . thermostat_suggestions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn current_thermostat_suggestion_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < ThermostatSuggestionStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . current_thermostat_suggestion (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn thermostat_suggestion_not_following_reason_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . thermostat_suggestion_not_following_reason (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThermostatAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thermostat_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn hvac_system_type_configuration_write (self , endpoint : crate :: dm :: EndptId , value : HVACSystemTypeBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . hvac_system_type_configuration (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn local_temperature_calibration_write (self , endpoint : crate :: dm :: EndptId , value : i8 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . local_temperature_calibration (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn occupied_cooling_setpoint_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . occupied_cooling_setpoint (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn occupied_heating_setpoint_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . occupied_heating_setpoint (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn unoccupied_cooling_setpoint_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . unoccupied_cooling_setpoint (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn unoccupied_heating_setpoint_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . unoccupied_heating_setpoint (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn min_heat_setpoint_limit_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . min_heat_setpoint_limit (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn max_heat_setpoint_limit_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . max_heat_setpoint_limit (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn min_cool_setpoint_limit_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . min_cool_setpoint_limit (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn max_cool_setpoint_limit_write (self , endpoint : crate :: dm :: EndptId , value : i16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . max_cool_setpoint_limit (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn min_setpoint_dead_band_write (self , endpoint : crate :: dm :: EndptId , value : i8 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . min_setpoint_dead_band (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn remote_sensing_write (self , endpoint : crate :: dm :: EndptId , value : RemoteSensingBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . remote_sensing (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn control_sequence_of_operation_write (self , endpoint : crate :: dm :: EndptId , value : ControlSequenceOfOperationEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . control_sequence_of_operation (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn system_mode_write (self , endpoint : crate :: dm :: EndptId , value : SystemModeEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . system_mode (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_write (self , endpoint : crate :: dm :: EndptId , value : TemperatureSetpointHoldEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . temperature_setpoint_hold (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn temperature_setpoint_hold_duration_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . temperature_setpoint_hold_duration (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn thermostat_programming_operation_mode_write (self , endpoint : crate :: dm :: EndptId , value : ProgrammingOperationModeBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . thermostat_programming_operation_mode (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn occupied_setback_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . occupied_setback (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn unoccupied_setback_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . unoccupied_setback (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn emergency_heat_delta_write (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . emergency_heat_delta (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_type_write (self , endpoint : crate :: dm :: EndptId , value : ACTypeEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_type (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_capacity_write (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_capacity (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_refrigerant_type_write (self , endpoint : crate :: dm :: EndptId , value : ACRefrigerantTypeEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_refrigerant_type (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_compressor_type_write (self , endpoint : crate :: dm :: EndptId , value : ACCompressorTypeEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_compressor_type (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_error_code_write (self , endpoint : crate :: dm :: EndptId , value : ACErrorCodeBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_error_code (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_louver_position_write (self , endpoint : crate :: dm :: EndptId , value : ACLouverPositionEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_louver_position (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn ac_capacityformat_write (self , endpoint : crate :: dm :: EndptId , value : ACCapacityFormatEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThermostatAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . thermostat_write () . ac_capacityformat (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `Thermostat` cluster. `use` this trait to call `.thermostat()` on any [`crate::im::ImClient`], such as an exchange; the returned [`ThermostatClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `ThermostatClientView` keeps each cluster's API surface narrow: at the call site `exchange.thermostat().` shows only this cluster's methods in IDE completion." ] pub trait ThermostatClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn thermostat (self ) -> ThermostatClientView < Self > {
        ThermostatClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > ThermostatClient for T {
    
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
    fn local_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn outdoor_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupancy (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OccupancyBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn abs_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn abs_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn abs_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn abs_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn pi_cooling_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn pi_heating_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn hvac_system_type_configuration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < HVACSystemTypeBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn local_temperature_calibration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_setpoint_dead_band (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn remote_sensing (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RemoteSensingBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn control_sequence_of_operation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ControlSequenceOfOperationEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn system_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < SystemModeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn thermostat_running_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThermostatRunningModeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn start_of_week (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < StartOfWeekEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_weekly_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_daily_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn temperature_setpoint_hold (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TemperatureSetpointHoldEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ProgrammingOperationModeBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn thermostat_running_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RelayStateBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn setpoint_change_source (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < SetpointChangeSourceEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn setpoint_change_amount (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn setpoint_change_source_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn occupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn unoccupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn unoccupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn unoccupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn emergency_heat_delta (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACTypeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_refrigerant_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACRefrigerantTypeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_compressor_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACCompressorTypeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_error_code (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACErrorCodeBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_louver_position (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACLouverPositionEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_coil_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn ac_capacityformat (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACCapacityFormatEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn preset_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetTypeStructArrayBuilder < P > , PresetTypeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn schedule_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleTypeStructArrayBuilder < P > , ScheduleTypeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_presets (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_schedules (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_schedule_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn number_of_schedule_transition_per_day (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_preset_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_schedule_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn presets < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetStructArrayBuilder < P > , PresetStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn schedules < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleStructArrayBuilder < P > , ScheduleStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn setpoint_hold_expiry_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_thermostat_suggestions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn thermostat_suggestions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ThermostatSuggestionStructArrayBuilder < P > , ThermostatSuggestionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ThermostatSuggestionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn thermostat_suggestion_not_following_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_hvac_system_type_configuration (& self , ctx : impl crate :: dm :: WriteContext , value : HVACSystemTypeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_local_temperature_calibration (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_occupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_min_setpoint_dead_band (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_remote_sensing (& self , ctx : impl crate :: dm :: WriteContext , value : RemoteSensingBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_control_sequence_of_operation (& self , ctx : impl crate :: dm :: WriteContext , value : ControlSequenceOfOperationEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_system_mode (& self , ctx : impl crate :: dm :: WriteContext , value : SystemModeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_temperature_setpoint_hold (& self , ctx : impl crate :: dm :: WriteContext , value : TemperatureSetpointHoldEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: WriteContext , value : ProgrammingOperationModeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_occupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_unoccupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_emergency_heat_delta (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_capacity (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_refrigerant_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACRefrigerantTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_compressor_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACCompressorTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_error_code (& self , ctx : impl crate :: dm :: WriteContext , value : ACErrorCodeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_louver_position (& self , ctx : impl crate :: dm :: WriteContext , value : ACLouverPositionEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_ac_capacityformat (& self , ctx : impl crate :: dm :: WriteContext , value : ACCapacityFormatEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_presets (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , PresetStruct < '_ > > , PresetStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_schedules (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ScheduleStruct < '_ > > , ScheduleStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_setpoint_raise_lower (& self , ctx : impl crate :: dm :: InvokeContext , request : SetpointRaiseLowerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , request : SetWeeklyScheduleRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_get_weekly_schedule < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetWeeklyScheduleRequest < '_ > , response : GetWeeklyScheduleResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_clear_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_active_schedule_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActiveScheduleRequestRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_active_preset_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActivePresetRequestRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_add_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddThermostatSuggestionRequest < '_ > , response : AddThermostatSuggestionResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_remove_thermostat_suggestion (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveThermostatSuggestionRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_atomic_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AtomicRequestRequest < '_ > , response : AtomicResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn local_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        T :: local_temperature (self , ctx ) 
    }
    fn outdoor_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        T :: outdoor_temperature (self , ctx ) 
    }
    fn occupancy (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OccupancyBitmap , crate :: error :: Error >> {
        T :: occupancy (self , ctx ) 
    }
    fn abs_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: abs_min_heat_setpoint_limit (self , ctx ) 
    }
    fn abs_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: abs_max_heat_setpoint_limit (self , ctx ) 
    }
    fn abs_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: abs_min_cool_setpoint_limit (self , ctx ) 
    }
    fn abs_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: abs_max_cool_setpoint_limit (self , ctx ) 
    }
    fn pi_cooling_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: pi_cooling_demand (self , ctx ) 
    }
    fn pi_heating_demand (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: pi_heating_demand (self , ctx ) 
    }
    fn hvac_system_type_configuration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < HVACSystemTypeBitmap , crate :: error :: Error >> {
        T :: hvac_system_type_configuration (self , ctx ) 
    }
    fn local_temperature_calibration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> {
        T :: local_temperature_calibration (self , ctx ) 
    }
    fn occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: occupied_cooling_setpoint (self , ctx ) 
    }
    fn occupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: occupied_heating_setpoint (self , ctx ) 
    }
    fn unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: unoccupied_cooling_setpoint (self , ctx ) 
    }
    fn unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: unoccupied_heating_setpoint (self , ctx ) 
    }
    fn min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: min_heat_setpoint_limit (self , ctx ) 
    }
    fn max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: max_heat_setpoint_limit (self , ctx ) 
    }
    fn min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: min_cool_setpoint_limit (self , ctx ) 
    }
    fn max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i16 , crate :: error :: Error >> {
        T :: max_cool_setpoint_limit (self , ctx ) 
    }
    fn min_setpoint_dead_band (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < i8 , crate :: error :: Error >> {
        T :: min_setpoint_dead_band (self , ctx ) 
    }
    fn remote_sensing (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RemoteSensingBitmap , crate :: error :: Error >> {
        T :: remote_sensing (self , ctx ) 
    }
    fn control_sequence_of_operation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ControlSequenceOfOperationEnum , crate :: error :: Error >> {
        T :: control_sequence_of_operation (self , ctx ) 
    }
    fn system_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < SystemModeEnum , crate :: error :: Error >> {
        T :: system_mode (self , ctx ) 
    }
    fn thermostat_running_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThermostatRunningModeEnum , crate :: error :: Error >> {
        T :: thermostat_running_mode (self , ctx ) 
    }
    fn start_of_week (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < StartOfWeekEnum , crate :: error :: Error >> {
        T :: start_of_week (self , ctx ) 
    }
    fn number_of_weekly_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_weekly_transitions (self , ctx ) 
    }
    fn number_of_daily_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_daily_transitions (self , ctx ) 
    }
    fn temperature_setpoint_hold (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TemperatureSetpointHoldEnum , crate :: error :: Error >> {
        T :: temperature_setpoint_hold (self , ctx ) 
    }
    fn temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: temperature_setpoint_hold_duration (self , ctx ) 
    }
    fn thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ProgrammingOperationModeBitmap , crate :: error :: Error >> {
        T :: thermostat_programming_operation_mode (self , ctx ) 
    }
    fn thermostat_running_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RelayStateBitmap , crate :: error :: Error >> {
        T :: thermostat_running_state (self , ctx ) 
    }
    fn setpoint_change_source (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < SetpointChangeSourceEnum , crate :: error :: Error >> {
        T :: setpoint_change_source (self , ctx ) 
    }
    fn setpoint_change_amount (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        T :: setpoint_change_amount (self , ctx ) 
    }
    fn setpoint_change_source_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: setpoint_change_source_timestamp (self , ctx ) 
    }
    fn occupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: occupied_setback (self , ctx ) 
    }
    fn occupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: occupied_setback_min (self , ctx ) 
    }
    fn occupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: occupied_setback_max (self , ctx ) 
    }
    fn unoccupied_setback (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: unoccupied_setback (self , ctx ) 
    }
    fn unoccupied_setback_min (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: unoccupied_setback_min (self , ctx ) 
    }
    fn unoccupied_setback_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: unoccupied_setback_max (self , ctx ) 
    }
    fn emergency_heat_delta (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: emergency_heat_delta (self , ctx ) 
    }
    fn ac_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACTypeEnum , crate :: error :: Error >> {
        T :: ac_type (self , ctx ) 
    }
    fn ac_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: ac_capacity (self , ctx ) 
    }
    fn ac_refrigerant_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACRefrigerantTypeEnum , crate :: error :: Error >> {
        T :: ac_refrigerant_type (self , ctx ) 
    }
    fn ac_compressor_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACCompressorTypeEnum , crate :: error :: Error >> {
        T :: ac_compressor_type (self , ctx ) 
    }
    fn ac_error_code (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACErrorCodeBitmap , crate :: error :: Error >> {
        T :: ac_error_code (self , ctx ) 
    }
    fn ac_louver_position (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACLouverPositionEnum , crate :: error :: Error >> {
        T :: ac_louver_position (self , ctx ) 
    }
    fn ac_coil_temperature (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i16 > , crate :: error :: Error >> {
        T :: ac_coil_temperature (self , ctx ) 
    }
    fn ac_capacityformat (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ACCapacityFormatEnum , crate :: error :: Error >> {
        T :: ac_capacityformat (self , ctx ) 
    }
    fn preset_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetTypeStructArrayBuilder < P > , PresetTypeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: preset_types (self , ctx , builder ) 
    }
    fn schedule_types < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleTypeStructArrayBuilder < P > , ScheduleTypeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: schedule_types (self , ctx , builder ) 
    }
    fn number_of_presets (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_presets (self , ctx ) 
    }
    fn number_of_schedules (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_schedules (self , ctx ) 
    }
    fn number_of_schedule_transitions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_schedule_transitions (self , ctx ) 
    }
    fn number_of_schedule_transition_per_day (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: number_of_schedule_transition_per_day (self , ctx ) 
    }
    fn active_preset_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_preset_handle (self , ctx , builder ) 
    }
    fn active_schedule_handle < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_schedule_handle (self , ctx , builder ) 
    }
    fn presets < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < PresetStructArrayBuilder < P > , PresetStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: presets (self , ctx , builder ) 
    }
    fn schedules < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ScheduleStructArrayBuilder < P > , ScheduleStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: schedules (self , ctx , builder ) 
    }
    fn setpoint_hold_expiry_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: setpoint_hold_expiry_timestamp (self , ctx ) 
    }
    fn max_thermostat_suggestions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: max_thermostat_suggestions (self , ctx ) 
    }
    fn thermostat_suggestions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ThermostatSuggestionStructArrayBuilder < P > , ThermostatSuggestionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: thermostat_suggestions (self , ctx , builder ) 
    }
    fn current_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ThermostatSuggestionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: current_thermostat_suggestion (self , ctx , builder ) 
    }
    fn thermostat_suggestion_not_following_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < ThermostatSuggestionNotFollowingReasonBitmap > , crate :: error :: Error >> {
        T :: thermostat_suggestion_not_following_reason (self , ctx ) 
    }
    fn set_hvac_system_type_configuration (& self , ctx : impl crate :: dm :: WriteContext , value : HVACSystemTypeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_hvac_system_type_configuration (self , ctx , value ) 
    }
    fn set_local_temperature_calibration (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_local_temperature_calibration (self , ctx , value ) 
    }
    fn set_occupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_occupied_cooling_setpoint (self , ctx , value ) 
    }
    fn set_occupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_occupied_heating_setpoint (self , ctx , value ) 
    }
    fn set_unoccupied_cooling_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_unoccupied_cooling_setpoint (self , ctx , value ) 
    }
    fn set_unoccupied_heating_setpoint (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_unoccupied_heating_setpoint (self , ctx , value ) 
    }
    fn set_min_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_min_heat_setpoint_limit (self , ctx , value ) 
    }
    fn set_max_heat_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_max_heat_setpoint_limit (self , ctx , value ) 
    }
    fn set_min_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_min_cool_setpoint_limit (self , ctx , value ) 
    }
    fn set_max_cool_setpoint_limit (& self , ctx : impl crate :: dm :: WriteContext , value : i16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_max_cool_setpoint_limit (self , ctx , value ) 
    }
    fn set_min_setpoint_dead_band (& self , ctx : impl crate :: dm :: WriteContext , value : i8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_min_setpoint_dead_band (self , ctx , value ) 
    }
    fn set_remote_sensing (& self , ctx : impl crate :: dm :: WriteContext , value : RemoteSensingBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_remote_sensing (self , ctx , value ) 
    }
    fn set_control_sequence_of_operation (& self , ctx : impl crate :: dm :: WriteContext , value : ControlSequenceOfOperationEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_control_sequence_of_operation (self , ctx , value ) 
    }
    fn set_system_mode (& self , ctx : impl crate :: dm :: WriteContext , value : SystemModeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_system_mode (self , ctx , value ) 
    }
    fn set_temperature_setpoint_hold (& self , ctx : impl crate :: dm :: WriteContext , value : TemperatureSetpointHoldEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_temperature_setpoint_hold (self , ctx , value ) 
    }
    fn set_temperature_setpoint_hold_duration (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_temperature_setpoint_hold_duration (self , ctx , value ) 
    }
    fn set_thermostat_programming_operation_mode (& self , ctx : impl crate :: dm :: WriteContext , value : ProgrammingOperationModeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_thermostat_programming_operation_mode (self , ctx , value ) 
    }
    fn set_occupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_occupied_setback (self , ctx , value ) 
    }
    fn set_unoccupied_setback (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_unoccupied_setback (self , ctx , value ) 
    }
    fn set_emergency_heat_delta (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_emergency_heat_delta (self , ctx , value ) 
    }
    fn set_ac_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_type (self , ctx , value ) 
    }
    fn set_ac_capacity (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_capacity (self , ctx , value ) 
    }
    fn set_ac_refrigerant_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACRefrigerantTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_refrigerant_type (self , ctx , value ) 
    }
    fn set_ac_compressor_type (& self , ctx : impl crate :: dm :: WriteContext , value : ACCompressorTypeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_compressor_type (self , ctx , value ) 
    }
    fn set_ac_error_code (& self , ctx : impl crate :: dm :: WriteContext , value : ACErrorCodeBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_error_code (self , ctx , value ) 
    }
    fn set_ac_louver_position (& self , ctx : impl crate :: dm :: WriteContext , value : ACLouverPositionEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_louver_position (self , ctx , value ) 
    }
    fn set_ac_capacityformat (& self , ctx : impl crate :: dm :: WriteContext , value : ACCapacityFormatEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_ac_capacityformat (self , ctx , value ) 
    }
    fn set_presets (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , PresetStruct < '_ > > , PresetStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_presets (self , ctx , value ) 
    }
    fn set_schedules (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ScheduleStruct < '_ > > , ScheduleStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_schedules (self , ctx , value ) 
    }
    fn handle_setpoint_raise_lower (& self , ctx : impl crate :: dm :: InvokeContext , request : SetpointRaiseLowerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_setpoint_raise_lower (self , ctx , request ) 
    }
    fn handle_set_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , request : SetWeeklyScheduleRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_weekly_schedule (self , ctx , request ) 
    }
    fn handle_get_weekly_schedule < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetWeeklyScheduleRequest < '_ > , response : GetWeeklyScheduleResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_get_weekly_schedule (self , ctx , request , response ) 
    }
    fn handle_clear_weekly_schedule (& self , ctx : impl crate :: dm :: InvokeContext , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_clear_weekly_schedule (self , ctx ) 
    }
    fn handle_set_active_schedule_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActiveScheduleRequestRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_active_schedule_request (self , ctx , request ) 
    }
    fn handle_set_active_preset_request (& self , ctx : impl crate :: dm :: InvokeContext , request : SetActivePresetRequestRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_active_preset_request (self , ctx , request ) 
    }
    fn handle_add_thermostat_suggestion < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddThermostatSuggestionRequest < '_ > , response : AddThermostatSuggestionResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_add_thermostat_suggestion (self , ctx , request , response ) 
    }
    fn handle_remove_thermostat_suggestion (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveThermostatSuggestionRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_remove_thermostat_suggestion (self , ctx , request ) 
    }
    fn handle_atomic_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AtomicRequestRequest < '_ > , response : AtomicResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_atomic_request (self , ctx , request , response ) 
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
                    AttributeId :: LocalTemperature => {
                        let attr_read_result = self . 0. local_temperature (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OutdoorTemperature => {
                        let attr_read_result = self . 0. outdoor_temperature (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OutdoorTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Occupancy => {
                        let attr_read_result = self . 0. occupancy (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Occupancy , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMinHeatSetpointLimit => {
                        let attr_read_result = self . 0. abs_min_heat_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMinHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMaxHeatSetpointLimit => {
                        let attr_read_result = self . 0. abs_max_heat_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMaxHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMinCoolSetpointLimit => {
                        let attr_read_result = self . 0. abs_min_cool_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMinCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AbsMaxCoolSetpointLimit => {
                        let attr_read_result = self . 0. abs_max_cool_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AbsMaxCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PICoolingDemand => {
                        let attr_read_result = self . 0. pi_cooling_demand (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PICoolingDemand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PIHeatingDemand => {
                        let attr_read_result = self . 0. pi_heating_demand (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PIHeatingDemand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HVACSystemTypeConfiguration => {
                        let attr_read_result = self . 0. hvac_system_type_configuration (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HVACSystemTypeConfiguration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalTemperatureCalibration => {
                        let attr_read_result = self . 0. local_temperature_calibration (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperatureCalibration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedCoolingSetpoint => {
                        let attr_read_result = self . 0. occupied_cooling_setpoint (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedCoolingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedHeatingSetpoint => {
                        let attr_read_result = self . 0. occupied_heating_setpoint (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedHeatingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedCoolingSetpoint => {
                        let attr_read_result = self . 0. unoccupied_cooling_setpoint (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedCoolingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedHeatingSetpoint => {
                        let attr_read_result = self . 0. unoccupied_heating_setpoint (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedHeatingSetpoint , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinHeatSetpointLimit => {
                        let attr_read_result = self . 0. min_heat_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxHeatSetpointLimit => {
                        let attr_read_result = self . 0. max_heat_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxHeatSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinCoolSetpointLimit => {
                        let attr_read_result = self . 0. min_cool_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxCoolSetpointLimit => {
                        let attr_read_result = self . 0. max_cool_setpoint_limit (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxCoolSetpointLimit , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinSetpointDeadBand => {
                        let attr_read_result = self . 0. min_setpoint_dead_band (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinSetpointDeadBand , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemoteSensing => {
                        let attr_read_result = self . 0. remote_sensing (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemoteSensing , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ControlSequenceOfOperation => {
                        let attr_read_result = self . 0. control_sequence_of_operation (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ControlSequenceOfOperation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SystemMode => {
                        let attr_read_result = self . 0. system_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SystemMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatRunningMode => {
                        let attr_read_result = self . 0. thermostat_running_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatRunningMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartOfWeek => {
                        let attr_read_result = self . 0. start_of_week (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartOfWeek , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfWeeklyTransitions => {
                        let attr_read_result = self . 0. number_of_weekly_transitions (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfWeeklyTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfDailyTransitions => {
                        let attr_read_result = self . 0. number_of_daily_transitions (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfDailyTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TemperatureSetpointHold => {
                        let attr_read_result = self . 0. temperature_setpoint_hold (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHold , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TemperatureSetpointHoldDuration => {
                        let attr_read_result = self . 0. temperature_setpoint_hold_duration (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHoldDuration , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatProgrammingOperationMode => {
                        let attr_read_result = self . 0. thermostat_programming_operation_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatProgrammingOperationMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatRunningState => {
                        let attr_read_result = self . 0. thermostat_running_state (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatRunningState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeSource => {
                        let attr_read_result = self . 0. setpoint_change_source (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeSource , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeAmount => {
                        let attr_read_result = self . 0. setpoint_change_amount (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeAmount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SetpointChangeSourceTimestamp => {
                        let attr_read_result = self . 0. setpoint_change_source_timestamp (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointChangeSourceTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetback => {
                        let attr_read_result = self . 0. occupied_setback (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetback , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetbackMin => {
                        let attr_read_result = self . 0. occupied_setback_min (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetbackMin , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OccupiedSetbackMax => {
                        let attr_read_result = self . 0. occupied_setback_max (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetbackMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetback => {
                        let attr_read_result = self . 0. unoccupied_setback (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetback , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetbackMin => {
                        let attr_read_result = self . 0. unoccupied_setback_min (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetbackMin , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UnoccupiedSetbackMax => {
                        let attr_read_result = self . 0. unoccupied_setback_max (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetbackMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EmergencyHeatDelta => {
                        let attr_read_result = self . 0. emergency_heat_delta (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EmergencyHeatDelta , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACType => {
                        let attr_read_result = self . 0. ac_type (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCapacity => {
                        let attr_read_result = self . 0. ac_capacity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACRefrigerantType => {
                        let attr_read_result = self . 0. ac_refrigerant_type (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACRefrigerantType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCompressorType => {
                        let attr_read_result = self . 0. ac_compressor_type (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCompressorType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACErrorCode => {
                        let attr_read_result = self . 0. ac_error_code (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACErrorCode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACLouverPosition => {
                        let attr_read_result = self . 0. ac_louver_position (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACLouverPosition , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCoilTemperature => {
                        let attr_read_result = self . 0. ac_coil_temperature (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCoilTemperature , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ACCapacityformat => {
                        let attr_read_result = self . 0. ac_capacityformat (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacityformat , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PresetTypes => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. preset_types (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PresetTypes , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ScheduleTypes => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. schedule_types (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScheduleTypes , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NumberOfPresets => {
                        let attr_read_result = self . 0. number_of_presets (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPresets , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfSchedules => {
                        let attr_read_result = self . 0. number_of_schedules (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfSchedules , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfScheduleTransitions => {
                        let attr_read_result = self . 0. number_of_schedule_transitions (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfScheduleTransitions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfScheduleTransitionPerDay => {
                        let attr_read_result = self . 0. number_of_schedule_transition_per_day (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfScheduleTransitionPerDay , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActivePresetHandle => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_preset_handle (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePresetHandle , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveScheduleHandle => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_schedule_handle (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveScheduleHandle , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Presets => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. presets (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Schedules => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. schedules (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SetpointHoldExpiryTimestamp => {
                        let attr_read_result = self . 0. setpoint_hold_expiry_timestamp (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SetpointHoldExpiryTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxThermostatSuggestions => {
                        let attr_read_result = self . 0. max_thermostat_suggestions (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxThermostatSuggestions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThermostatSuggestions => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. thermostat_suggestions (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestions , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentThermostatSuggestion => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_thermostat_suggestion (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentThermostatSuggestion , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ThermostatSuggestionNotFollowingReason => {
                        let attr_read_result = self . 0. thermostat_suggestion_not_following_reason (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatSuggestionNotFollowingReason , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: HVACSystemTypeConfiguration => {
                let attr_data : HVACSystemTypeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_hvac_system_type_configuration (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HVACSystemTypeConfiguration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalTemperatureCalibration => {
                let attr_data : i8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_temperature_calibration (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalTemperatureCalibration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedCoolingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_cooling_setpoint (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedCoolingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedHeatingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_heating_setpoint (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedHeatingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedCoolingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_cooling_setpoint (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedCoolingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedHeatingSetpoint => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_heating_setpoint (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedHeatingSetpoint , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinHeatSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_heat_setpoint_limit (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinHeatSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MaxHeatSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_max_heat_setpoint_limit (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxHeatSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinCoolSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_cool_setpoint_limit (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinCoolSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MaxCoolSetpointLimit => {
                let attr_data : i16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_max_cool_setpoint_limit (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxCoolSetpointLimit , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MinSetpointDeadBand => {
                let attr_data : i8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_min_setpoint_dead_band (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinSetpointDeadBand , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: RemoteSensing => {
                let attr_data : RemoteSensingBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_remote_sensing (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemoteSensing , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ControlSequenceOfOperation => {
                let attr_data : ControlSequenceOfOperationEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_control_sequence_of_operation (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ControlSequenceOfOperation , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SystemMode => {
                let attr_data : SystemModeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_system_mode (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SystemMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: TemperatureSetpointHold => {
                let attr_data : TemperatureSetpointHoldEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_temperature_setpoint_hold (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHold , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: TemperatureSetpointHoldDuration => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_temperature_setpoint_hold_duration (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TemperatureSetpointHoldDuration , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ThermostatProgrammingOperationMode => {
                let attr_data : ProgrammingOperationModeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_thermostat_programming_operation_mode (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThermostatProgrammingOperationMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OccupiedSetback => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_occupied_setback (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OccupiedSetback , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: UnoccupiedSetback => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_unoccupied_setback (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UnoccupiedSetback , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: EmergencyHeatDelta => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_emergency_heat_delta (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EmergencyHeatDelta , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACType => {
                let attr_data : ACTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_type (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCapacity => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_capacity (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacity , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACRefrigerantType => {
                let attr_data : ACRefrigerantTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_refrigerant_type (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACRefrigerantType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCompressorType => {
                let attr_data : ACCompressorTypeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_compressor_type (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCompressorType , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACErrorCode => {
                let attr_data : ACErrorCodeBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_error_code (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACErrorCode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACLouverPosition => {
                let attr_data : ACLouverPositionEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_louver_position (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACLouverPosition , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ACCapacityformat => {
                let attr_data : ACCapacityFormatEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_ac_capacityformat (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ACCapacityformat , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Presets => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_presets (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Presets , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Schedules => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_schedules (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Schedules , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: SetpointRaiseLower => {
                let cmd_data : SetpointRaiseLowerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_setpoint_raise_lower (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetpointRaiseLower ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetWeeklySchedule => {
                let cmd_data : SetWeeklyScheduleRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_weekly_schedule (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetWeeklySchedule ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GetWeeklySchedule => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (0u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_get_weekly_schedule (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetWeeklySchedule ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ClearWeeklySchedule => {
                let cmd_invoke_result = self . 0. handle_clear_weekly_schedule (& ctx ) . await ;
                crate :: dm :: log_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ClearWeeklySchedule ) ) ) , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetActiveScheduleRequest => {
                let cmd_data : SetActiveScheduleRequestRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_active_schedule_request (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetActiveScheduleRequest ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetActivePresetRequest => {
                let cmd_data : SetActivePresetRequestRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_active_preset_request (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetActivePresetRequest ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: AddThermostatSuggestion => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_thermostat_suggestion (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddThermostatSuggestion ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveThermostatSuggestion => {
                let cmd_data : RemoveThermostatSuggestionRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_remove_thermostat_suggestion (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveThermostatSuggestion ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: AtomicRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (253u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_atomic_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AtomicRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 513u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Thermostat" , 513u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Thermostat" , 513u32 , self . 0.2 ) 
    }
    
}
