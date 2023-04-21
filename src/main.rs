
use rocksdb::{DB, Options, IteratorMode};
fn main(){

    let key ="name".to_string();
    let value ="dharamendra".to_string();
    
    set(key.clone(),value);
    get_with_key(key);
    get();
     
}
const path:&str = "./db";


fn set(key:String, value:String) -> Result<(), Box<dyn std::error::Error>> {
    let value: Vec<u8> = value.into_bytes();
    let key: Vec<u8> = key.into_bytes();
    let db = DB::open_default(path).unwrap();
    db.put(key, value)?;
    Ok(())
}

fn get() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open_default(path).unwrap();
    let iter = db.iterator(IteratorMode::Start);
       for item in iter {
         let (key, value) = item.unwrap();
         println!("Saw {:?} {:?}",String::from_utf8_lossy(&key).to_string(),String::from_utf8_lossy(&value).to_string());
     }
    Ok(())
}
fn get_with_key(key:String){
    let key: Vec<u8> = key.into_bytes();
    let db = DB::open_default(path).unwrap();
    match db.get(key) {
               Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
               Ok(None) => println!("value not found"),
               Err(e) => println!("operational problem encountered: {}", e),
           }
}

