use crate::cache::ResponseCache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::rc::Rc;

pub trait Emeter {
    fn get_emeter_realtime(&mut self) -> Result<RealtimeStats>;
    fn get_emeter_month_stats(&mut self, year: u32) -> Result<MonthStats>;
    fn get_emeter_day_stats(&mut self, month: u32, year: u32) -> Result<DayStats>;
    fn erase_emeter_stats(&mut self) -> Result<()>;
}

pub(crate) struct EmeterStats {
    ns: String,
    proto: Rc<Proto>,
    cache: Rc<ResponseCache>,
}

impl EmeterStats {
    pub(crate) fn new(ns: &str, proto: Rc<Proto>, cache: Rc<ResponseCache>) -> EmeterStats {
        EmeterStats {
            ns: String::from(ns),
            proto,
            cache,
        }
    }

    pub(crate) fn get_realtime(&self) -> Result<RealtimeStats> {
        let request = Request::new(&self.ns, "get_realtime", None);

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .try_get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("({}) {:?}", self.ns, response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }

    pub(crate) fn get_day_stats(&self, month: u32, year: u32) -> Result<DayStats> {
        let request = Request::new(
            &self.ns,
            "get_daystat",
            Some(json!({ "month": month , "year": year})),
        );

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .try_get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("({}) {:?}", self.ns, response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }

    pub(crate) fn get_month_stats(&self, year: u32) -> Result<MonthStats> {
        let request = Request::new(&self.ns, "get_monthstat", Some(json!({ "year": year })));

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .try_get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("({}) {:?}", self.ns, response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }

    pub(crate) fn erase_stats(&self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let response =
            self.proto
                .send_request(&Request::new(&self.ns, "erase_emeter_stat", None))?;

        log::debug!("{:?}", response);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeStats {
    #[serde(flatten)]
    stats: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayStats {
    day_list: Vec<DayStat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayStat {
    energy_wh: u32,
    day: u32,
    month: u32,
    year: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthStats {
    month_list: Vec<MonthStat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MonthStat {
    energy_wh: u32,
    month: u32,
    year: u32,
}
