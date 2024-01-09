extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::{env, io};

fn main() -> Result<()> {
    // SQLiteデータベースに接続
    let conn = Connection::open("todo.db")?;
    // "todo.db" はデータベースファイル名
    // 存在しない場合は新規作成される

    // テーブルが存在しない場合のみ作成する
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            done INTEGER NOT NULL
        )",
        [],
    )?;

    if let Ok(current_dir) = env::current_dir() {
        println!("Current directory: {:?}", current_dir);
    }

    println!("Welcome to the Todo App!");
    loop {
        println!("1. タスクの一覧を表示");
        println!("2. タスクを追加");
        println!("3. タスクを完了にする");
        println!("4. タスクを削除");
        println!("5. アプリケーションを終了");
        println!("選択肢を入力してください (1-5):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("入力エラー");
        match choice.trim() {
            "1" => display_tasks(&conn)?,
            "2" => add_task(&conn)?,
            "3" => complete_task(&conn)?,
            "4" => delete_task(&conn)?,
            "5" => {
                println!("アプリケーションを終了します。");
                break; // ループを抜けてアプリケーションを終了
            }
            _ => {
                println!("無効な選択です。");
            }
        }
    }

    Ok(())
}

fn display_tasks(conn: &Connection) -> Result<()> {
    println!("タスク一覧:");

    let mut stmt = conn.prepare("SELECT id, title, description, done FROM todo")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i32>(3)?,
        ))
    })?;

    for task in task_iter {
        if let Ok((id, title, description, done)) = task {
            let status = if done == 1 { "完了" } else { "未完了" };
            println!(
                "ID: {}, タイトル: {}, 詳細: {}, ステータス: {}",
                id, title, description, status
            );
        }
    }

    Ok(())
}

fn add_task(conn: &Connection) -> Result<()> {
    println!("新しいタスクを追加します。");

    println!("タイトル:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("タイトルの読み込みに失敗しました");
    title = title.trim().to_string(); // 改行コードを削除

    println!("詳細:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("詳細の読み込みに失敗しました");
    description = description.trim().to_string(); // 改行コードを削除

    // タスクをデータベースに保存する
    conn.execute(
        "INSERT INTO todo (title, description, done) VALUES (?1, ?2, 0)",
        &[&title, &description],
    )?;

    println!("タスクが追加されました。");

    Ok(())
}

fn complete_task(conn: &Connection) -> Result<()> {
    // タスクの一覧を表示して、完了するタスクを選択
    println!("完了するタスクを選択してください:");
    display_tasks(&conn)?;

    println!("完了するタスクのIDを入力してください:");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("タスクIDの読み込みに失敗しました");

    let task_id: i32 = match task_id.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("無効なタスクIDです。");
            return Ok(());
        }
    };

    // タスクを完了状態に更新する
    conn.execute("UPDATE todo SET done = 1 WHERE id = ?1", &[&task_id])?;

    println!("タスクが完了しました。");

    Ok(())
}

fn delete_task(conn: &Connection) -> Result<()> {
    // タスクの一覧を表示して、削除するタスクを選択
    println!("削除するタスクを選択してください:");
    display_tasks(&conn)?;

    println!("削除するタスクのIDを入力してください:");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("タスクIDの読み込みに失敗しました");

    let task_id: i32 = match task_id.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("無効なタスクIDです。");
            return Ok(());
        }
    };

    // タスクを削除する
    conn.execute("DELETE FROM todo WHERE id = ?1", &[&task_id])?;

    println!("タスクが削除されました。");

    Ok(())
}
