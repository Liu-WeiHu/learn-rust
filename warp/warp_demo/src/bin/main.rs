use warp_demo::{ models, filters };
use warp::Filter;

#[tokio::main]
async fn main() {

    let db = models::new_db();
    let routes = filters::list_sims(db.clone())
        .or(filters::post_sim(db.clone()))
        .or(filters::update_sim(db.clone()))
        .or(filters::delete_sim(db));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/*
curl --location --request POST 'localhost:3030/holodeck' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "id": 1,
    "name": "The Big Goodbye"
}'

curl --location --request POST 'localhost:3030/holodeck' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "id": 2,
    "name": "Bride Of Chaotica!"
}'

curl --location --request PUT 'localhost:3030/holodeck/3' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "A Fistful Of Datas"
}'

curl --location --request PUT 'localhost:3030/holodeck/3' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "A Fistful Of La Forges"
}'


curl --location --request GET 'localhost:3030/holodeck'

curl --location --request GET 'localhost:3030/holodeck/2'

curl --location --request DELETE 'localhost:3030/holodeck/1' 

*/