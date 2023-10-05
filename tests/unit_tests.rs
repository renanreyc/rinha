// use rinha_backend_rust;
// use warp::Filter;


// #[tokio::test]
// async fn test_count_people() {
//     let route = warp::path("contagem-pessoas")
//     .and(warp::get())
//     .and_then(count_people);

//     // Crie uma solicitação de teste GET para a rota /contagem-pessoas/
//     let resp = warp::test::request()
//         .method("GET")
//         .path("/contagem-pessoas/")
//         .reply(&route)
//         .await;

//     // Verifique se a função count_people foi chamada imprimindo "hello count people"
//     assert!(resp.body().contains("hello count people"));

// }