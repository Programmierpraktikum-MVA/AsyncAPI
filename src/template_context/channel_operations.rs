use crate::asyncapi_model::AsyncAPI;

use super::{utilities, SimplifiedOperation};

pub fn get_subscribe_channels_operations(
    asyncapi: &AsyncAPI,
) -> Vec<(&String, SimplifiedOperation)> {
    asyncapi
        .channels
        .iter()
        .filter_map(|(channel_name, channel)| {
            channel.subscribe.as_ref().map(|operation| {
                (
                    channel_name,
                    utilities::simplify_operation(operation, channel_name),
                )
            })
        })
        .collect()
}
pub fn get_publish_channels_operations(asyncapi: &AsyncAPI) -> Vec<(&String, SimplifiedOperation)> {
    asyncapi
        .channels
        .iter()
        .filter_map(|(channel_name, channel)| {
            channel.publish.as_ref().map(|operation| {
                (
                    channel_name,
                    utilities::simplify_operation(operation, channel_name),
                )
            })
        })
        .collect()
}
