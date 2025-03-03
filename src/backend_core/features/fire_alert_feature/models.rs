use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SensorLogData {
    pub id: u32,
    pub component: u32,
    pub value: f32,
    pub alert: FireStatus,
    pub timestamp: SystemTime,
}

pub enum SensorDataType {
    Fire,
    Smoke,
    CO,
    Heat,
    FireButton,
    FireLight,
    FireBuzzer,
    LPG,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, JsonSchema, Clone, Copy, Debug)]
#[repr(u8)]
pub enum FireStatus {
    SAFE = 0,
    UNSAFE = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FireLog {
    pub owner_name: String,
    pub fire_logs: Vec<SensorLogData>,
    pub smoke_logs: Vec<SensorLogData>,
    pub co_logs: Vec<SensorLogData>,
    pub heat_logs: Vec<SensorLogData>,
    pub button_logs: Vec<SensorLogData>,
    pub light_logs: Vec<SensorLogData>,
    pub buzzer_logs: Vec<SensorLogData>,
    pub lpg_logs: Vec<SensorLogData>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Pagination {
    pub start_time: Option<i32>,
    pub end_time: Option<i32>,
    pub offset: Option<u32>,
    pub limit: Option<i64>,
}
