use tide::convert::Deserialize;
use tide::convert::Serialize;
#[derive(Debug, Deserialize, Serialize)]
struct Dino {
    name: String,
    weight: u16,
    diet: String
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    use tide::Request;
    use tide::Response;
    use tide::Body;
    
    tide::log::start();

    let mut app = tide::new();
    
    app.at("/").get(|_| async {
        Ok("Hello, world!")
    });
    
    app.at("/dinos").post(|mut req: Request<()>| async move {
        let dino: Dino = req.body_json().await?;
        println!("{:?}", dino);
        let mut res = Response::new(201);
        res.set_body(Body::from_json(&dino)?);
        Ok(res)
    });
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}