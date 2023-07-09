use clap::Parser;
use crate::{model::*, utils::*};
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client, Message};

/// specify Messages to send using your new Microservice!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///Command to either send all example messages specified in the spec or a single custom message
    /// use with --c all to send all example messages (potential custom messages will be ignored)
    /// use with --c destination/channel -message {myMessageJson} to send a custom message to a specific destination/channel 
    #[arg(short, long, default_value = "" )]
    pub command: String,

    ///specify the message according to the specified message schema
    #[arg(short, long, default_value="" )]
    pub message: String,
}



pub async fn send_all_messages(client: &Client)-> Result<(), async_nats::Error>{
	//TODO: modify this template to iterate over .subscribe channels so that they are sent to their respective channels
    Ok(())

}

pub async fn handle_cli(client: &Client, command: &String, message: &String)-> Result<(), async_nats::Error> {
    match command.as_str(){
        "all" => {
            send_all_messages(&client).await?;
        },
        "" => {
                ();
        },
        _ => {
            client.publish(command.into(), message.to_owned().into()).await?;
            println!("Sent message {:?} to {}",message, command);
        }
    }
    Ok(())
}