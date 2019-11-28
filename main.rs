pub struct NewsArticle{
   pub author:String,
   pub content:String,
}
pub struct Tweet{
    username:String,
    content:String,
}
pub trait Summary{
    fn summarize(&self)->String;
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("NewsArticle: {} by {}",self.content,self.author)
    }
}
impl Summary for Tweet{
    fn summarize(&self)->String{
      format!("Tweet: {} by {}",self.content,self.username)
    }
}
fn main(){
    let article=NewsArticle{
        author:String::from("Ibad Rauf Kz "),
        content:String::from("Role Of Muslims"),
    };
    let tweet=Tweet{
        username:String::from("Kapil"),
        content:String::from("For Supporting Him"),
    };
    println!("{}",tweet.summarize());
    println!("{}",article.summarize());
}
