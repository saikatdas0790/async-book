use futures::executor;

#[derive(Debug)]
struct Song;

async fn learn_song() -> Song {
    return Song;
}

async fn sing_song(song: Song) {
    println!("Singing {:?}", song);
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    executor::block_on(async_main());
}
