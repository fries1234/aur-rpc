use crate::{info, search, search_by, SearchField};

#[tokio::test]
async fn it_searches_by_name() {
    let packages = search("yay").await.unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_searches_by_maintainer() {
    let packages = search_by(SearchField::Maintainer, "trivernis")
        .await
        .unwrap();
    assert!(packages.len() > 0)
}

#[tokio::test]
async fn it_returns_information() {
    let packages = info(["yay"]).await.unwrap();
    assert!(packages.len() > 0)
}
