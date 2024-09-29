
mod aiagent;


use frankenstein::GetUpdatesParams;
use frankenstein::ReplyParameters;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use aiagent::ai_agent;



static TOKEN: &str = "7371234570:AAFgNG9mOWPs8z7Sz_7xTOwUEOJibf6LzH8";

fn main() {

    dotenv_flow::dotenv_flow().ok();
    let api = Api::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();



    loop {
        let result = api.get_updates(&update_params);

        println!("result: {result:?}");

        // Matching results from the api call: This result is either Ok() or Error()
        match result {
            Ok(response) => {

                // Looping throught all the object in the response body
                for update in response.result {

                    // Getting the message object from looped parameters
                    if let UpdateContent::Message(message) = update.content {

                        if let Some(text) = message.text.clone(){
                            println!("Text:{}",text);

                            let ai_response = ai_agent(&text);  


                           


                            // Abstracting the sent message instance for future reply
                        let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                       

                       

                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(&ai_response.unwrap())
                            .reply_parameters(reply_parameters)
                            .build();


                        // let send_message_params = SendMessageParams::builder()
                        //     .chat_id(message.chat.id)
                        //     .text(text)
                        //     .reply_parameters(reply_parameters)
                        //     .build();



                        if let Err(err) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {err:?}");
                        }
                            
                            
                        }





                        

                        
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            },
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
}