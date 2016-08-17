extern crate rusoto;

use rusoto::logs::{CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::logs::{DescribeLogStreamsRequest, LogStream};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::thread;

type LogGroupName = String;

struct LogGroupWatcher {
    map: HashMap<LogGroupName, Option<LogStream>>,
}

impl LogGroupWatcher {
    pub fn new() -> LogGroupWatcher {
        LogGroupWatcher {
            map: HashMap::new(),
        }
    }

    fn set_initial_map(&mut self) {
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = CloudWatchLogsClient::new(credentials, Region::UsEast1);
        for log_group_name in self.log_group_names() {
            let request = DescribeLogStreamsRequest {
                log_group_name: log_group_name.clone(),
                descending: Some(true),
                limit: Some(1),
                order_by: Some("LastEventTime".into()),
                ..Default::default()
            };
            let response = client.describe_log_streams(&request).unwrap();
            let alpha = response.log_streams.unwrap().into_iter().next();
            self.map.insert(log_group_name.clone(), alpha);

            // Slow down requests
            // https://github.com/rusoto/rusoto/issues/234
            sleep(Duration::from_millis(100));
        }
    }

    fn log_group_names(&mut self) -> Vec<LogGroupName> {
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = CloudWatchLogsClient::new(credentials, Region::UsEast1);
        let mut request = DescribeLogGroupsRequest::default();
        let mut log_group_names = vec![];

        loop {
            let response = client.describe_log_groups(&request).unwrap();
            for log_group in response.log_groups.unwrap() {
                let log_group_name = log_group.log_group_name.unwrap();
                log_group_names.push(log_group_name);
            }

            if response.next_token.is_some() {
                request.next_token = response.next_token;
            } else {
                break;
            }

            // Slow down requests
            // https://github.com/rusoto/rusoto/issues/234
            sleep(Duration::from_millis(100));
        }

        log_group_names
    }

    pub fn start_watching(&mut self) {
        self.set_initial_map();

        thread::spawn(|| {
            loop {
                sleep(Duration::from_secs(1));
                println!("from thread");
            }
        });
    }
}

fn main() {
    let mut log_group_watcher = LogGroupWatcher::new();
    log_group_watcher.start_watching();
    sleep(Duration::from_secs(100));
}
