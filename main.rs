fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel(); // создаем канал

    let handle = std::thread::spawn({
        // создаем поток
        let vs = vs.to_owned(); // копируем входные данные

        move || {
            for v in vs {
                // записываем элементы массива в канал с задержкой в 1 секунду
                tx.send(v).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            // закрываем отправитель
            drop(tx);
        }
    });
    // ожидаем завершения работы потока
    handle.join().unwrap();
    // возвращаем приемник
    rx
}

fn merge(
    a: std::sync::mpsc::Receiver<i32>,
    b: std::sync::mpsc::Receiver<i32>,
) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel(); // создаем канал

    let mut a_done = false; // Переменные сигнализирующие о том что в канале больше нет сообщений или канал был закрыт

    let mut b_done = false;

    loop {
        match a.try_recv() {
            // пытаемся получить сообщение из канала "a"
            Ok(i) => {
                // Сообщение успешно получено - записываем его в итоговый канал
                tx.send(i).unwrap();
            }

            Err(_) => {
                // Сообщение не получено или канал был закрыт
                a_done = true;
            }
        }

        match b.try_recv() {
            // пытаемся получить сообщение из канала "b"
            Ok(i) => {
                // Сообщение успешно получено - записываем его в итоговый канал
                tx.send(i).unwrap();
            }

            Err(_) => {
                // Сообщение не получено или канал был закрыт
                b_done = true;
            }
        }

        if a_done && b_done {
            // Выходим из цикла если были прочитаны все сообщения в обоих каналах
            break;
        }
    }
    // Возвращаем приемник итогового канала
    rx
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]); // Создаем каналы и записываем данные

    let b = as_chan(&vec![2, 4, 6, 8]);

    let c = merge(a, b); // Обьединяем данные из двух каналов в один

    for v in c.iter() {
        println!("{v:?}");
    }
}
