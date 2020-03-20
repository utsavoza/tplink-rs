use crate::cache::ResponseCache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::rc::Rc;
use std::time::Duration;

pub trait Timer {
    fn get_timer_rules(&mut self) -> Result<RuleList>;
    fn add_timer_rule(&mut self, rule: Rule) -> Result<String>;
    fn edit_timer_rule(&mut self, id: &str, rule: Rule) -> Result<()>;
    fn delete_timer_rule_with_id(&mut self, id: &str) -> Result<()>;
    fn delete_all_timer_rules(&mut self) -> Result<()>;
}

pub(crate) struct TimerSettings {
    ns: String,
    proto: Rc<Proto>,
    cache: Rc<ResponseCache>,
}

impl TimerSettings {
    pub(crate) fn new(ns: &str, proto: Rc<Proto>, cache: Rc<ResponseCache>) -> TimerSettings {
        TimerSettings {
            ns: String::from(ns),
            proto,
            cache,
        }
    }

    pub(crate) fn get_rules(&self) -> Result<RuleList> {
        let request = Request::new(&self.ns, "get_rules", None);

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("{:?}", response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }

    pub(crate) fn add_rule(&self, rule: Rule) -> Result<String> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let Rule {
            enable,
            delay,
            act,
            name,
            ..
        } = rule;

        let response = self.proto.send_request(&Request::new(
            &self.ns,
            "add_rule",
            Some(json!({"enable": enable, "delay": delay, "act": act, "name": name})),
        ))?;

        log::trace!("{:?}", response);

        Ok(response["id"].to_string())
    }

    pub(crate) fn edit_rule(&self, id: &str, rule: Rule) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let Rule {
            enable,
            delay,
            act,
            name,
            ..
        } = rule;

        let response = self.proto.send_request(&Request::new(
            &self.ns,
            "edit_rule",
            Some(json!({"id": id, "enable": enable, "delay": delay, "act": act, "name": name})),
        ))?;

        log::trace!("{:?}", response);

        Ok(())
    }

    pub(crate) fn delete_rule_with_id(&self, id: &str) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let response = self.proto.send_request(&Request::new(
            &self.ns,
            "delete_rule",
            Some(json!({ "id": id })),
        ))?;

        log::trace!("{:?}", response);

        Ok(())
    }

    pub(crate) fn delete_all_rules(&self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns);
        }

        let response =
            self.proto
                .send_request(&Request::new(&self.ns, "delete_all_rules", None))?;

        log::trace!("{:?}", response);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleList {
    rule_list: Vec<Rule>,
}

impl RuleList {
    pub fn len(&self) -> usize {
        self.rule_list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rule_list.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    // power state
    act: u32,
    // delay in secs
    delay: u64,
    // enable the rule
    enable: u32,
    // name of the rule
    name: String,
    // rule id (skip serializing if empty)
    id: Option<String>,
    // remaining time in secs (Skip serializing)
    remain: Option<i64>,
}

impl Rule {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    turn_on: bool,
    enable_rule: bool,
    delay: Duration,
    name: String,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            turn_on: true,
            enable_rule: true,
            delay: Duration::from_secs(1),
            name: String::from("timer"),
        }
    }

    pub fn turn_on(&mut self, turn_on: bool) -> &mut Builder {
        self.turn_on = turn_on;
        self
    }

    pub fn enable(&mut self, enable_rule: bool) -> &mut Builder {
        self.enable_rule = enable_rule;
        self
    }

    pub fn delay(&mut self, delay: Duration) -> &mut Builder {
        self.delay = delay;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Builder {
        self.name = String::from(name);
        self
    }

    pub fn build(&mut self) -> Rule {
        let act = if self.turn_on { 1 } else { 0 };
        let delay = self.delay.as_secs();
        let enable = if self.enable_rule { 1 } else { 0 };
        let name = self.name.to_string();

        Rule {
            act,
            delay,
            enable,
            name,
            id: None,
            remain: None,
        }
    }
}
