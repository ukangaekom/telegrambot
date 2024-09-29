
use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest, MessageContent};
use genai::Client;
// use genai::chat::printer::print_chat_stream;



#[tokio::main]

pub async fn ai_agent(_text:&str) -> Option<std::string::String>  {

    let client = Client::default();

    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system("Give a specialized answer like a crypto wizard"),
        ChatMessage::user(_text.to_string())
    ]);

    let model: &str = "gemini-1.5-flash-latest";

    let chat_res = client.exec_chat_stream(model, chat_req, None).await.ok();
    
    let reply = print_chat_stream(chat_res.expect("REASON"),  None).await.ok();

    return reply;



         

    // match chat_res{
    //     Ok(val) =>{
    //         println!(val);
    //     },

    //     Err(e) =>{
    //         println!("error {}",e);
    //     }
    }

   

