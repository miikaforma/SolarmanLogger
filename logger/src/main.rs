#[macro_use]
extern crate log;

use std::time::Duration;

use api::CurrentDataResponse;
use api::access_token;
use api::current_data;
use api::historical_data;
use chrono::{NaiveDate, DateTime, Utc};
use dotenv::dotenv;
use influxdb::Client;
use influxdb::InfluxDbWriteable;
use tokio::time::sleep;
use chrono_tz::Tz;

mod logging;

#[derive(InfluxDbWriteable)]
#[allow(non_snake_case)]
struct CurrentData {
    time: DateTime<Utc>,
    #[influxdb(tag)]
    device_sn: String,
    #[influxdb(tag)]
    device_id: u32,

    SN1: Option<String>,
    SS_CY1: Option<u16>,
    MODEi1: Option<u8>,
    HWv1: Option<String>,
    SWmai_v1: Option<String>,
    DSPv1: Option<String>,
    DSPv2: Option<String>,
    FUSE_V: Option<String>,
    DV1: Option<f32>,
    DV2: Option<f32>,
    DC1: Option<f32>,
    DC2: Option<f32>,
    DP1: Option<f32>,
    DP2: Option<f32>,
    AV1: Option<f32>,
    AV2: Option<f32>,
    AV3: Option<f32>,
    AC1: Option<f32>,
    AC2: Option<f32>,
    AC3: Option<f32>,
    A_Fo1: Option<f32>,
    APo_t1: Option<f32>,
    Et_ge0: Option<f32>,
    Etdy_ge1: Option<f32>,
    ST_PG1: Option<String>,
    PG_Pt1: Option<f32>,
    E_Puse_t1: Option<f32>,
    INV_T0: Option<f32>,
    T_MDU1: Option<f32>,
    SYSTIM1: Option<String>,
    t_w_hou1: Option<u32>,
    CD_TIM1: Option<u32>,
    Bus_V1: Option<f32>,
    Dcp1: Option<f32>,
    Dcp2: Option<f32>,
    Dcp3: Option<f32>,
    EAR_N_Ineg1: Option<f32>,
    CT_P1: Option<f32>,
    INV_ST1: Option<String>,
    N_I1: Option<f32>,
    N_I2: Option<f32>,
}


async fn fetch_historical_data(client: &Client, access_token: String, device_sn: String, device_id: String, start_time: String, end_time: String) {
    info!("Fetching historical data for {} between {} - {}", &device_sn, &start_time, &end_time);

    match historical_data(access_token, device_sn, device_id, start_time, end_time).await {
        Ok(data) => {
            for param_data_list in data.param_data_list.iter() {

                let mut device_state = 1;
                let mut iter = param_data_list.data_list.iter();
                let status = iter.find(| &x| x.key == "INV_ST1").unwrap();
                if status.value.as_ref().unwrap_or(&"".to_string()).contains("Standby") {
                    device_state = 3;
                }

                log_new_entry(client, &CurrentDataResponse {
                    code: None,
                    msg: None,
                    success: data.success,
                    request_id: data.request_id.clone(),
                    device_sn: data.device_sn.clone(),
                    device_id: data.device_id,
                    device_type: data.device_type.clone(),
                    device_state: device_state,
                    data_list: param_data_list.data_list.clone(),
                }).await
            }
        }
        Err(_) => {}
    }
}

async fn fetch_and_log_new_entry(client: &Client, access_token: String, device_sn: String, device_id: String) {
    info!("Logging new entry for device {}", &device_sn);

    match current_data(access_token, device_sn, device_id).await {
        Ok(data) => {
            log_new_entry(client, &data).await
        }
        Err(_) => {}
    }
}

async fn log_new_entry(client: &Client, data: &CurrentDataResponse) {
    let mut iter = data.data_list.iter();
    let system_time = iter.find(| &x| x.key == "SYSTIM1");

    if system_time.is_none() {
        warn!("Skipping logging because system time couldn't be found");
        return;
    }

    let system_time = system_time.unwrap().to_utc_datetime();
    if system_time.is_none() {
        warn!("Skipping logging because system time couldn't be parsed");
        return;
    }

    let system_time = system_time.unwrap();
    info!("System Time UTC: {:?}", system_time);
    let mut current_data = CurrentData {
        time: system_time,

        device_sn: data.device_sn.clone(),
        device_id: data.device_id,

        SN1: None,
        SS_CY1: None,
        MODEi1: None,
        HWv1: None,
        SWmai_v1: None,
        DSPv1: None,
        DSPv2: None,
        FUSE_V: None,

        DV1: None,
        DV2: None,
        DC1: None,
        DC2: None,
        DP1: None,
        DP2: None,
        AV1: None,
        AV2: None,
        AV3: None,
        AC1: None,
        AC2: None,
        AC3: None,
        A_Fo1: None,
        APo_t1: None,
        Et_ge0: None,
        Etdy_ge1: None,
        ST_PG1: None,
        PG_Pt1: None,
        E_Puse_t1: None,
        INV_T0: None,
        T_MDU1: None,
        SYSTIM1: None,
        t_w_hou1: None,
        CD_TIM1: None,
        Bus_V1: None,
        Dcp1: None,
        Dcp2: None,
        Dcp3: None,
        EAR_N_Ineg1: None,
        CT_P1: None,
        INV_ST1: None,
        N_I1: None,
        N_I2: None,
    };

    for data_list in data.data_list.iter() {
        let key = data_list.key.as_str();

        match key {
            "SN1" => {
                current_data.SN1 = data_list.to_string();
                // println!("SN: {:?}", current_data.SN1);
            }
            "SS_CY1" => {
                current_data.SS_CY1 = data_list.to_u16();
                // println!("Production Compliance Country: {:?}", current_data.SS_CY1);
            }
            "MODEi1" => { current_data.MODEi1 = data_list.to_u8(); }
            "HWv1" => { current_data.HWv1 = data_list.to_string(); }
            "SWmai_v1" => { current_data.SWmai_v1 = data_list.to_string(); }
            "DSPv1" => { current_data.DSPv1 = data_list.to_string(); }
            "DSPv2" => { current_data.DSPv2 = data_list.to_string(); }
            "FUSE_V" => { current_data.FUSE_V = data_list.to_string(); }
            "DV1" => { current_data.DV1 = data_list.to_float(); }
            "DV2" => { current_data.DV2 = data_list.to_float(); }
            "DC1" => { current_data.DC1 = data_list.to_float(); }
            "DC2" => { current_data.DC2 = data_list.to_float(); }
            "DP1" => { current_data.DP1 = data_list.to_float(); }
            "DP2" => { current_data.DP2 = data_list.to_float(); }
            "AV1" => { current_data.AV1 = data_list.to_float(); }
            "AV2" => { current_data.AV2 = data_list.to_float(); }
            "AV3" => { current_data.AV3 = data_list.to_float(); }
            "AC1" => { current_data.AC1 = data_list.to_float(); }
            "AC2" => { current_data.AC2 = data_list.to_float(); }
            "AC3" => { current_data.AC3 = data_list.to_float(); }
            "A_Fo1" => { current_data.A_Fo1 = data_list.to_float(); }
            "APo_t1" => { current_data.APo_t1 = data_list.to_float(); }
            "Et_ge0" => { current_data.Et_ge0 = data_list.to_float(); }
            "Etdy_ge1" => { current_data.Etdy_ge1 = data_list.to_float(); }
            "ST_PG1" => { current_data.ST_PG1 = data_list.to_string(); }
            "PG_Pt1" => { current_data.PG_Pt1 = data_list.to_float(); }
            "E_Puse_t1" => {
                current_data.E_Puse_t1 = data_list.to_float();
                // println!("Total Consumption Power: {:?}", current_data.E_Puse_t1);
            }
            "INV_T0" => { current_data.INV_T0 = data_list.to_float(); }
            "T_MDU1" => { current_data.T_MDU1 = data_list.to_float(); }
            "SYSTIM1" => { current_data.SYSTIM1 = data_list.to_string(); }
            "t_w_hou1" => { current_data.t_w_hou1 = data_list.to_u32(); }
            "CD_TIM1" => { current_data.CD_TIM1 = data_list.to_u32(); }
            "Bus_V1" => { current_data.Bus_V1 = data_list.to_float(); }
            "Dcp1" => { current_data.Dcp1 = data_list.to_float(); }
            "Dcp2" => { current_data.Dcp2 = data_list.to_float(); }
            "Dcp3" => { current_data.Dcp3 = data_list.to_float(); }
            "EAR_N_Ineg1" => { current_data.EAR_N_Ineg1 = data_list.to_float(); }
            "CT_P1" => { current_data.CT_P1 = data_list.to_float(); }
            "INV_ST1" => { current_data.INV_ST1 = data_list.to_string(); }
            "N_I1" => { current_data.N_I1 = data_list.to_float(); }
            "N_I2" => { current_data.N_I2 = data_list.to_float(); }
            _ => {}
        }
    }

    let write_result = client.query(&current_data.into_query("currentData")).await;
    if let Err(err) = write_result {
        eprintln!("Error writing to db: {}", err)
    }
}

async fn get_access_token(app_id: u64, app_secret: String, email: String, password: String) -> String {
    info!("Fetching a new access_token for app {}", &app_id);

    match access_token(app_id, app_secret, email, password).await {
        Ok(data) => {
            if !data.success {
                panic!("Error fetching a new token {} - {}", data.code.unwrap_or("".to_string()), data.msg.unwrap_or("".to_string()));
            }

            data.access_token.unwrap()
        }
        Err(err) => panic!("Token fetch request failed {}", err)
    }
}

fn get_timezone() -> Tz {
    let timezone = dotenv::var("CHRONO_TIMEZONE").unwrap_or("Europe/Helsinki".to_string());
    timezone.parse().unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    logging::init_logging();

    info!("SolarmanLogger starting");
    info!("Using time zone: {}", get_timezone().name());

    let database_url = dotenv::var("DATABASE_URL").unwrap_or("http://localhost:8086".to_string());
    let database_name = dotenv::var("DATABASE_NAME").unwrap_or("solarman".to_string());
    let interval: u64 = dotenv::var("INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(60_000))
        .unwrap();

    let email = dotenv::var("EMAIL").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();
    let app_secret = dotenv::var("APP_SECRET").unwrap();
    let app_id: u64 = dotenv::var("APP_ID")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(0))
        .unwrap();

    let device_sn = dotenv::var("DEVICE_SN").unwrap();
    let device_id = dotenv::var("DEVICE_ID").unwrap();

    let insert_historical_data: bool = dotenv::var("INSERT_HISTORICAL_DATA")
        .map(|var| var.parse::<bool>())
        .unwrap_or(Ok(false))
        .unwrap();

    // Connect to database
    let client = Client::new(database_url, database_name);

    // Get auth token
    let access_token = get_access_token(app_id, app_secret, email, password)
        .await;

    if insert_historical_data
    {
        let start_time = dotenv::var("START_TIME").unwrap();
        let end_time = dotenv::var("END_TIME").unwrap();

        let naive_start_time = NaiveDate::parse_from_str(&start_time, "%Y-%m-%d").unwrap();
        let naive_end_time = NaiveDate::parse_from_str(&end_time, "%Y-%m-%d").unwrap();

        let duration = naive_end_time.signed_duration_since(naive_start_time).num_days();
        for n in 0..duration + 1 {
            let dt = naive_start_time + chrono::Duration::days(n);

            fetch_historical_data(&client, access_token.to_string(), device_sn.clone(), device_id.clone(), dt.format("%Y-%m-%d").to_string(), dt.format("%Y-%m-%d").to_string()).await;
        }
    }

    // println!("Starting fetch loop for current data for {} with the interval of {}", &device_sn, interval);
    loop {
        fetch_and_log_new_entry(&client, access_token.to_string(), device_sn.clone(), device_id.clone()).await;
        sleep(Duration::from_millis(interval)).await;
    }
}
