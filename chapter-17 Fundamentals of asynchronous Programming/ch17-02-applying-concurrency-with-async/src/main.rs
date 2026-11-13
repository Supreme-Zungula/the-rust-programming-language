use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        handle.await.unwrap();

        //--------------------------------------------------

        println!("\nUsing the trpl::join method on async futures:");
        let fut1 = async {
            for i in 1..10 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Hi number {i} for the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;

        // Sending Data between two tasks using message passing.
        // -----------------------------------------------------
        println!("\nSending Data between two tasks using message passing.");
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }

        // Code withing one async block executes linearly.
        // ----------------------------------------------
        println!("\n Code within one async block executes linearly");

        let tx1 = tx.clone();

        // moving ownership of tx1 to async block
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // Move ownership of tx to the async block
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
