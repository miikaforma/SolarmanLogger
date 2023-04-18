use serde::{Deserialize, Serialize};
use chrono::TimeZone;
use chrono::{NaiveDateTime, DateTime, Utc};
// use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentDataRequest {
    pub device_sn: String,
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataList {
    pub key: String,
    pub value: Option<String>,
    pub unit: Option<String>,
    pub name: Option<String>,
}

impl DataList {
    fn get_timezone() -> Tz {
        let timezone = dotenv::var("CHRONO_TIMEZONE").unwrap_or("Europe/Helsinki".to_string());
        timezone.parse().unwrap()
    }

    pub fn to_utc_datetime(&self) -> Option<DateTime<Utc>> {
        let naive_time = NaiveDateTime::parse_from_str(&self.value.as_ref().unwrap(), "%y-%m-%d %H:%M:%S");
        if naive_time.is_err() {
            return None;
        }
        // println!("System Time EEST {}", naive_time);

        // Some(Utc.from_utc_datetime(&naive_time.unwrap()))
        Some(Utc.from_utc_datetime(&DataList::get_timezone().from_local_datetime(&naive_time.unwrap())
            .unwrap()
            .naive_utc()))
    }

    pub fn to_string(&self) -> Option<String> {
        if self.value.is_none() {
            warn!("Tried to convert none value to String for key {}", self.key);
            None
        }
        else {
            let value = self.value.as_ref().unwrap();
            if let Ok(val) = value.parse::<String>() {
                // println!("Converted key {} to String {}", self.key, val);
                Some(val)
            }
            else {
                error!("Parsing key {} to String failed", self.key);
                None
            }
        }
    }


    pub fn to_float(&self) -> Option<f32> {
        if self.value.is_none() {
            warn!("Tried to convert none value to float for key {}", self.key);
            None
        }
        else {
            let value = self.value.as_ref().unwrap();
            if let Ok(val) = value.parse::<f32>() {
                // println!("Converted key {} to float {}", self.key, val);
                Some(val)
            }
            else {
                error!("Parsing key {} to float failed", self.key);
                None
            }
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        if self.value.is_none() {
            warn!("Tried to convert none value to u32 for key {}", self.key);
            None
        }
        else {
            let value = self.value.as_ref().unwrap();
            if let Ok(val) = value.parse::<u32>() {
                // println!("Converted key {} to u32 {}", self.key, val);
                Some(val)
            }
            else {
                error!("Parsing key {} to u32 failed", self.key);
                None
            }
        }
    }

    pub fn to_u16(&self) -> Option<u16> {
        if self.value.is_none() {
            warn!("Tried to convert none value to u16 for key {}", self.key);
            None
        }
        else {
            let value = self.value.as_ref().unwrap();
            if let Ok(val) = value.parse::<u16>() {
                // println!("Converted key {} to u16 {}", self.key, val);
                Some(val)
            }
            else {
                error!("Parsing key {} to u16 failed", self.key);
                None
            }
        }
    }

    pub fn to_u8(&self) -> Option<u8> {
        if self.value.is_none() {
            warn!("Tried to convert none value to u8 for key {}", self.key);
            None
        }
        else {
            let value = self.value.as_ref().unwrap();
            if let Ok(val) = value.parse::<u8>() {
                // println!("Converted key {} to u8 {}", self.key, val);
                Some(val)
            }
            else {
                error!("Parsing key {} to u8 failed", self.key);
                None
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentDataResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub success: bool,
    pub request_id: String,
    pub device_sn: String,
    pub device_id: u32,
    pub device_type: String,
    pub device_state: u8,
    pub data_list: Vec<DataList>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDataRequest {
    pub device_sn: String,
    pub device_id: String,
    pub start_time: String,
    pub end_time: String,
    pub time_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDataResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub success: bool,
    pub request_id: String,
    pub device_sn: String,
    pub device_id: u32,
    pub device_type: String,
    pub time_type: u8,
    pub param_data_list: Vec<ParamDataList>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParamDataList {
    pub collect_time: String,
    pub data_list: Vec<DataList>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    pub app_secret: String,
    pub email: String,
    pub password: String,
    pub org_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub success: bool,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
    pub scope: Option<String>,
    pub uid: Option<u32>,
}
