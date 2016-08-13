extern crate rusoto;

use rusoto::logs::{CloudWatchLogsClient, DescribeLogGroupsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::logs::{LogGroupName, DescribeLogStreamsRequest, LogGroup, GetLogEventsRequest};
use std::collection::HashMap;

fn aws_lambda_log_group_names() -> Vec<LogGroupName> {
    let mut log_groups: Vec<LogGroupName> = vec![];

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchLogsClient::new(credentials, Region::UsEast1);
    let mut request = DescribeLogGroupsRequest::default();

    loop {
        let response = client.describe_log_groups(&request).unwrap();
        let a: Vec<LogGroup> = response.log_groups.unwrap();
        let mut b: Vec<LogGroupName> = vec![];
        for i in a {
            b.push(i.log_group_name.unwrap());
        }
        log_groups.extend(b);

        if response.next_token.is_some() {
            request.next_token = response.next_token;
        } else {
            break;
        }
    }

    for log_group_name in &log_groups {
        let request = DescribeLogStreamsRequest {
            log_group_name: log_group_name.clone(),
            ..Default::default()
        };

        let response = client.describe_log_streams(&request).unwrap();

        for i in response.log_streams.unwrap() {
            println!("{}", i.log_stream_name.unwrap());
        }
    }
    
    log_groups
}

fn main() {
    let log_groups = aws_lambda_log_group_names();
    for log_group in log_groups {
        //println!("{}", log_group);
    }
}
