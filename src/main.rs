extern crate rusoto;

use rusoto::logs::{CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::logs::{LogGroupName, DescribeLogStreamsRequest, LogGroup, GetLogEventsRequest, LogStream};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

struct LogGroupWatcher {
    map: HashMap<String, Option<LogStream>>,
}

impl LogGroupWatcher {
    fn new() -> LogGroupWatcher {
        let mut log_groups: Vec<LogGroupName> = vec![];

        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = CloudWatchLogsClient::new(credentials, Region::UsEast1);
        let mut request = DescribeLogGroupsRequest::default();

        let mut map = HashMap::new();

        loop {
            let response = client.describe_log_groups(&request).unwrap();
            let a: Vec<LogGroup> = response.log_groups.unwrap();
            for i in a {
                let log_group_name = i.log_group_name.unwrap();
                let request = DescribeLogStreamsRequest {
                    log_group_name: log_group_name.clone(),
                    descending: Some(true),
                    limit: Some(1),
                    order_by: Some("LastEventTime".into()),
                    ..Default::default()
                };
                let response = client.describe_log_streams(&request).unwrap();
                let alpha = response.log_streams.unwrap().into_iter().next();
                map.insert(log_group_name.clone(), alpha);

                // Slow down requests
                // https://github.com/rusoto/rusoto/issues/234
                sleep(Duration::from_millis(100));
            }

            if response.next_token.is_some() {
                request.next_token = response.next_token;
            } else {
                break;
            }
        }

        LogGroupWatcher {
            map: map,
        }
    }
}

fn main() {
    let log_group_watcher = LogGroupWatcher::new();
}
