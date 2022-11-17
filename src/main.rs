use twitter_v2;
use slugify_rs::slugify;
use std::process::Command;
use random_number::random;
use std::fs::read_to_string;


#[tokio::main]
async fn main() {

    set_up().await
}

struct YtInputs {
    yt_title : String,
    yt_links : Vec<String>,
}

async fn set_up() {

    let bearer_token = read_to_string("/home/alexc/projects/rust/testing/bearer_token.txt").unwrap();
    
    let auth = twitter_v2::authorization::BearerToken::new(format!("{}", bearer_token.trim()));
    let user = twitter_v2::TwitterApi::new(auth.clone())
                .get_user_by_username(String::from("outoftheway4"))
                .send()
                .await.expect("this shouldnt work but i think it does")
                .into_data()
                .expect("this tweet should exist");

    
    let mut liked_tweets = twitter_v2::TwitterApi::new(auth.clone())
                       .get_user_liked_tweets(user.id)
                       .send()
                       .await.expect("thehe");
    

    while liked_tweets.meta.as_ref().unwrap().next_token != None {
        for i in 0..liked_tweets.data.as_ref().unwrap().len() {
            downloader(cut_and_cop(&liked_tweets.data.as_ref().unwrap()[i].text));
        }
        if liked_tweets.meta.as_ref().unwrap().result_count > 0 {
            liked_tweets = twitter_v2::TwitterApi::new(auth.clone())
                           .get_user_liked_tweets(user.id)
                           .pagination_token(liked_tweets.meta.as_ref().unwrap().next_token.as_ref().unwrap())
                           .send()
                           .await.expect("thehe");
            
        } 
        
    }
}



fn cut_and_cop(input: &str) -> YtInputs {
    let mut link_list: Vec<String> = vec![];
    let mut text_assc = String::from(input);
    let link_source = text_assc.clone();
        while text_assc.find("https://t.co/").is_some() == true {
            match text_assc.find("https://t.co/") {
                Some(spot) => { 
                    let link_item = &link_source[spot as usize..(spot as usize + 23)];
                    link_list.push(String::from(link_item));
                    text_assc.replace_range(spot as usize..(spot as usize + 23), ""); 
                              },
                None => println!("Link Not Found"),
            }
    }
    let yt_inputs = YtInputs {
        yt_title : slugify!(&text_assc, separator = " "),
        yt_links : link_list,
    };
    yt_inputs
}

fn downloader(value: YtInputs) {
        

    for link in value.yt_links.iter() {
        let n: u32 = random!(..99999);
        let mut test_result = value.yt_title.clone();
        if test_result.chars().nth(0).is_none() == true {
            test_result = format!("[{}]", n);
        } else if test_result.len() > 6 {
            test_result = format!("{}[{}]", &value.yt_title.clone()[..(value.yt_title.len() - 7)], n);
        } else {
            test_result = format!("{}[{}]", &value.yt_title.clone(), n);
        }
        let mut yt_download = Command::new("yt-dlp")
                            .arg(format!("-o{}.%(ext)s", test_result))
                            .arg(link)
                            .spawn()
                            .expect("you typed something wrong");

        yt_download.wait().expect("this will burn your cpu if it goes wrong");
    }
}