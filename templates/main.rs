use async_nats::{Client, Subscriber};
use futures::StreamExt;

async fn listen_for_message(sub1: &mut Subscriber) {
    while let Some(message) = sub1.next().await {
        println!("Received message {:#?}", message);
    }
}

async fn publish_messages(client: &Client, amount: i32, channel: &str, data: &'static str) {
    for _ in 0..amount {
        client.publish(channel.into(), data.into()).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ server.url }}").await?;
    // declare subscribers
    {% for sub_channel in subscribe_channels %}
    let mut {{ sub_channel.1.operation_id }} = client.subscribe("{{ sub_channel.0  }}".into()).await?;
    {% endfor %}

    // publish
    tokio::join!(
    {% for pub_channel in publish_channels %}
        publish_messages(&client, 10, "{{ pub_channel.0  }}", "test payload"),
    {% endfor %});

    // subscribe
    tokio::select! {
    {% for sub_channel in subscribe_channels %}
        _=listen_for_message(&mut {{ sub_channel.1.operation_id  }}) => {}
    {% endfor %}
    }
    println!("fin");

    Ok(())
}
