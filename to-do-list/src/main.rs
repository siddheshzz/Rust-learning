use fancy::printcoln;
use postgres::types::Type;
use postgres::{error, Client, Error, NoTls};
use std::collections::HashMap;
use std::io::{self, Stdin};
use std::env;
use dotenv::dotenv;

struct Task {
    _id: i32,
    name: String,
}

fn read_from_console() -> String {
    let mut buff = String::new();

    let _ = io::stdin().read_line(&mut buff);
    return buff;
}
fn print(client: &mut Client) -> Result<(), Error> {
    println!("\n\n");
    println!("<=== To Do List ===>");
    for row in client.query("SELECT id, name FROM todo", &[])? {
        let task = Task {
            _id: row.get(0),
            name: row.get(1),
        };
        println!("{}   {} ", task._id, task.name);
    }
    println!("\n");
    Ok(())
}

fn input(client: &mut Client) -> Result<(), Error> {

    // let mut list = Vec::new();
    let taskName = read_from_console().trim().to_owned();
    let task = Task {
        _id: 0,
        name: taskName,
    };
    client.execute("INSERT INTO todo (name) VALUES ($1)", &[&task.name])?;
    Ok(())
}

fn mark_done(client: &mut Client) ->Result<(),Error> {
    println!("Enter the task number to mark done ✅ or undone");
   
    let task_number = (read_from_console()
        .trim()
        .to_owned()
        .parse::<i32>()
        .unwrap())
        ;
        let res = client.query("SELECT id,name FROM todo WHERE id=($1)", &[&task_number])?;
        if res.len() == 0 || res.len() > 1 {
            println!("The given task number does not exist please try again with correct number");
            return Ok(()) ;
        } else {
            let mut id_no=0;
            let mut task = Task{
                _id:0,
                name:"".to_string(),
            };
            for row in res {
                task._id = row.get(0);
                task.name = row.get(1);
            }
            if task_number == task._id {
                if task.name.contains("✅"){
                    task.name = task.name.trim_end_matches("✅").to_string();
                }else {
                    task.name = task.name+"✅";
                }
                println!("{}",task.name);
                
                let statement =
                    client.prepare_typed("INSERT INTO todo (id,name) VALUES ($1,$2) on conflict(id)
                    do
                    update set name=EXCLUDED.name", &[Type::INT4,Type::VARCHAR])?;
                let res = client.execute(&statement, &[&task._id,&task.name])?;
    
                println!("Operation was SUCCESFUL");
            }
            else {
                    println!("The given task number does not exist please try again with correct number");
                }
        }
        Ok(())


    // let mut s = list.get(indx.clone()).unwrap();
    // if s.contains("✅") {
    //     let x = s.trim_end_matches("✅");
    //     list.insert(indx, x.to_string());
    //     let _ = list.remove(indx + 1 as usize);
    // } else {
    //     let s = list.get(indx.clone()).unwrap();
    //     list.insert(indx, s.to_owned() + &"✅");
    //     let _ = list.remove(indx + 1);
    // }
    // s.push_str("✅");
}

fn delete_task(client: &mut Client) -> Result<(), Error> {
    println!("🚨Enter the task number to be deleted");
    let task_number = (read_from_console()
        .trim()
        .to_owned()
        .parse::<i32>()
        .unwrap())
        ;
    let res = client.query("SELECT id,name FROM todo WHERE id=($1)", &[&task_number])?;
    if res.len() == 0 || res.len() > 1 {
        println!("The given task number does not exist please try again with correct number");
        return Ok(()) ;
    } else {
        let mut id_no=0;
        for row in res {
            id_no = row.get(0);
        }
        if task_number == id_no {
            let statement =
                client.prepare_typed("DELETE FROM todo WHERE id = $1", &[Type::INT4])?;
            let res = client.execute(&statement, &[&id_no])?;

            println!("DELETED THE TASK SUCCESFULLY");
        }
        else {
                println!("The given task number does not exist please try again with correct number");
            }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    println!("<=== Welcome ===>");
    printcoln!("[bold|cyan]To DO[magenta]!");
    dotenv().ok();
    let params = std::env::var("DB_URL").expect("Check database URL creds should be---> 'postgresql://postgres:PASSWORD@localhost/userTest'");
    let mut client = Client::connect(&params, NoTls)?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS todo (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL
                )",
    )?;
    loop {
        println!("Select a choice:\n1. Add task\n2. Delete task\n3. Mark Unmark\n4. Show\n5. Exit");

        match read_from_console().as_str().trim() {
            "1" => input(&mut client),
            "2" => {
                print(&mut client);
                delete_task(&mut client);
                Ok(())
            }
            "3" =>mark_done(&mut client),
            "4" => print(&mut client),
            "5" => break,
            _ => Ok(println!("Please Enter a correct choice")),
        };
    }
    Ok(())
}
