pub mod models;

use http::{StatusCode, header::USER_AGENT, header::AUTHORIZATION};
use dotenv::dotenv;
pub use models::*;

pub async fn access_token(app_id: u64, app_secret: String, email: String, password: String) -> Result<TokenResponse, anyhow::Error> {
    let res = reqwest::Client::new()
        .post(format!("https://api.solarmanpv.com/account/v1.0/token?appId={}&language=en", app_id))
        .json(&TokenRequest {
            app_secret,
            email,
            password,
            org_id: None,
        })
        .header(USER_AGENT, "SolarmanLogger")
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: TokenResponse = serde_json::from_str(&data_str)?;
    // println!("TokenResponse: {:#?}", data);

    Ok(data)
}

pub async fn current_data(access_token: String, device_sn: String, device_id: String) -> Result<CurrentDataResponse, anyhow::Error> {
    current_data_with_lang(access_token, device_sn, device_id, "en".to_string()).await
}

pub async fn current_data_with_lang(access_token: String, device_sn: String, device_id: String, lang: String) -> Result<CurrentDataResponse, anyhow::Error> {
    let res = reqwest::Client::new()
        .post(format!("https://api.solarmanpv.com/device/v1.0/currentData?language={}", lang))
        .json(&CurrentDataRequest {
            device_sn,
            device_id,
        })
        .header(USER_AGENT, "SolarmanLogger")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: CurrentDataResponse = serde_json::from_str(&data_str)?;
    // println!("CurrentDataResponse: {:#?}", data);

    Ok(data)
}

pub async fn historical_data(access_token: String, device_sn: String, device_id: String, start_time: String, end_time: String) -> Result<HistoricalDataResponse, anyhow::Error> {
    historical_data_with_lang(access_token, device_sn, device_id, start_time, end_time, "en".to_string()).await
}

pub async fn historical_data_with_lang(access_token: String, device_sn: String, device_id: String, start_time: String, end_time: String, lang: String) -> Result<HistoricalDataResponse, anyhow::Error> {
    let res = reqwest::Client::new()
        .post(format!("https://api.solarmanpv.com/device/v1.0/historical?language={}", lang))
        .json(&HistoricalDataRequest {
            device_sn,
            device_id,
            start_time,
            end_time,
            time_type: 1,
        })
        .header(USER_AGENT, "SolarmanLogger")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: HistoricalDataResponse = serde_json::from_str(&data_str)?;
    // println!("HistoricalDataResponse: {:#?}", data);

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_access_token() {
        dotenv().ok();

        let email = dotenv::var("EMAIL").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();
        let app_secret = dotenv::var("APP_SECRET").unwrap();
        let app_id: u64 = dotenv::var("APP_ID")
            .map(|var| var.parse::<u64>())
            .unwrap_or(Ok(0))
            .unwrap();

        let response = access_token(app_id, app_secret, email, password).await.unwrap();
        assert_eq!(true, response.success);
    }

    #[tokio::test]
    async fn get_current_data() {
        dotenv().ok();

        let device_sn = dotenv::var("DEVICE_SN").unwrap();
        let device_id = dotenv::var("DEVICE_ID").unwrap();

        let email = dotenv::var("EMAIL").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();
        let app_secret = dotenv::var("APP_SECRET").unwrap();
        let app_id: u64 = dotenv::var("APP_ID")
            .map(|var| var.parse::<u64>())
            .unwrap_or(Ok(0))
            .unwrap();

        let access_token = access_token(app_id, app_secret, email, password)
            .await
            .unwrap()
            .access_token
            .unwrap();

        let response = current_data_with_lang(access_token, device_sn, device_id, "en".to_string()).await.unwrap();
        assert_eq!(true, response.success);
    }
}
