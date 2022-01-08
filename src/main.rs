use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::sync::Arc;
use tide::{Body, Request, Response};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Dino {
    name: String,
    weight: u16,
    diet: String,
}

#[derive(Clone, Debug)]
struct State {
    dinos: Arc<RwLock<HashMap<String, Dino>>>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let state = State {
        dinos: Default::default(),
    };

    let mut app = tide::with_state(state);

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.at("/dinos").post(|mut req: Request<State>| async move {
        let dino: Dino = req.body_json().await?;

        let mut dinos = req.state().dinos.write().await;
        dinos.insert(String::from(&dino.name), dino.clone());
        let mut res = Response::new(201);
        res.set_body(Body::from_json(&dino)?);
        Ok(res)
    });

    app.at("/dinos").get(|req: Request<State>| async move {
        let dinos = req.state().dinos.read().await;
        let dinos_vec: Vec<Dino> = dinos.values().cloned().collect();
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&dinos_vec)?);
        Ok(res)
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
