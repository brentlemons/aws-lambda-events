use crate::custom_serde::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AWS S3 event
///
/// [https://docs.aws.amazon.com/lambda/latest/dg/with-s3.html](https://docs.aws.amazon.com/lambda/latest/dg/with-s3.html)
/// [http://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-s3-put](http://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-s3-put)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Event {
    #[serde(rename = "Records")]
    pub records: Vec<S3Record>,
}

/// `S3EventRecord` which wrap record data
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Record {
    /// All examples available are only 2.0, but...
    ///
    /// Amazon S3 uses versions 2.1, 2.2, and 2.3 of this event structure. The below is according to [S3 User Guide](https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-content-structure.html)
    ///
    /// - 2.1 - all events not covered by 2.2 and 2.3
    /// - 2.2 - cross-Region replication event notifications
    /// - 2.3 - S3 Lifecycle, S3 Intelligent-Tiering, object ACL, object tagging, and object restoration delete events
    ///
    /// 2.2 and 2.3 contain extra information specific to these operations, but are otherwise compatible with 2.1
    pub event_version: String,

    /// The source of the event. Should be *aws:s3*
    pub event_source: String,

    /// The region the event originated from.
    pub aws_region: String,

    /// The time, in ISO-8601 format, for example, 1970-01-01T00:00:00.000Z, when Amazon S3 finished processing the request.
    pub event_time: DateTime<Utc>,

    /// event_name references the list of [event notification types](https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-event-types-and-destinations.html) but doesn't contain the s3: prefix
    pub event_name: String,

    ///
    pub user_identity: Identity,

    /// Currently only contains source IP of the change
    pub request_parameters: S3RequestParameters,

    /// The response_elements key value is useful if you want to trace a request by following up with AWS Support.
    /// Both x-amz-request-id and x-amz-id-2 help Amazon S3 trace an individual request. These values are the
    /// same as those that Amazon S3 returns in the response to the request that initiates the events. This is
    /// so they can be used to match the event to the request.
    pub response_elements: S3ResponseElements,

    ///
    pub s3: S3Entity,
    /* the following is from https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-content-structure.html, add?
     * The glacierEventData key is only visible for s3:ObjectRestore:Completed events.
     * The restoreEventData key contains attributes that are related to your restore request.
     * The replicationEventData key is only visible for replication events.
     * The intelligentTieringEventData key is only visible for S3 Intelligent-Tiering events.
     * The lifecycleEventData key is only visible for S3 Lifecycle transition events.
     */
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// Amazon customer-ID of the user who caused the event
    pub principal_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3RequestParameters {
    /// IP address where request came from
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3ResponseElements {
    /// Amazon S3 generated request ID
    #[serde(rename = "x-amz-request-id")]
    pub amazon_request_id: String,

    /// Amazon S3 host that processed the request
    #[serde(rename = "x-amz-id-2")]
    pub amazon_host_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Entity {
    #[serde(rename = "s3SchemaVersion")]
    pub schema_version: String,

    /// ID found in the bucket notification configuration
    pub configuration_id: String,

    /// The bucket
    pub bucket: S3Bucket,

    /// The object
    pub object: S3Object,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Bucket {
    /// The bucket name (globally unique)
    pub name: String,

    /// Amazon customer-ID of the bucket owner
    pub owner_identity: Identity,

    /// The bucket ARN
    pub arn: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Object {
    /// The object key
    pub key: String,

    /* This is not actually part of the message. Java calculates this: https://github.com/aws/aws-sdk-java/blob/6a4c873c71320ef0175ca1c13188e9c850a85e51/aws-java-sdk-s3/src/main/java/com/amazonaws/services/s3/event/S3EventNotification.java#L176-L183
    pub url_decoded_key: Option<String>, */
    /// The object size in bytes
    #[serde(default)]
    pub size: Option<i64>,

    /// object version if bucket is versioning-enabled, otherwise null
    // #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version_id: Option<String>,

    /// The object eTag??
    // #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub e_tag: Option<String>,

    /// The sequencer key provides a way to determine the sequence of events. Event notifications aren't guaranteed
    /// to arrive in the same order that the events occurred. However, notifications from events that create objects
    /// (PUTs) and delete objects contain a sequencer. It can be used to determine the order of events for a given
    /// object key.
    ///
    /// If you compare the sequencer strings from two event notifications on the same object key, the event notification
    /// with the greater sequencer hexadecimal value is the event that occurred later. If you're using event
    /// notifications to maintain a separate database or index of your Amazon S3 objects, we recommend that you compare
    /// and store the sequencer values as you process each event notification.
    ///
    /// Note the following:
    /// - You can't use sequencer to determine order for events on different object keys.
    /// - The sequencers can be of different lengths. So, to compare these values, first right pad the shorter value with zeros, and then do a lexicographical comparison.
    // #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sequencer: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "s3")]
    fn example_s3_event_objectcreated_put() {
        let data = include_bytes!("test_data/s3-event-objectcreated-put.json");
        let parsed: S3Event = serde_json::from_slice(data).unwrap();
        println!("--> {:?} <--", parsed);
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: S3Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "s3")]
    fn example_s3_event_objectcreated_put_versioning() {
        let data = include_bytes!("test_data/s3-event-objectcreated-put-versioning.json");
        let parsed: S3Event = serde_json::from_slice(data).unwrap();
        println!("--> {:?} <--", parsed);
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: S3Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "s3")]
    fn example_s3_event_objectremoved_delete() {
        let data = include_bytes!("test_data/s3-event-objectremoved-delete.json");
        let parsed: S3Event = serde_json::from_slice(data).unwrap();
        println!("--> {:?} <--", parsed);
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: S3Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
