# Rust File Handling

- Working with text file
- Working with JSON file

#### Working with text file
In Rust, you can read from and write to text files using the std::fs module and the std::io module. Below is an example of how to read from and write to a text file.

```File::open```: Opens the file in read-only mode.
```File::create```: Creates a new file or overwrites an existing file in write-only mode.
```read_to_string```: Reads the file’s contents into a String.
```write_all```: Writes content to a file.

Approaches to open and read file:-
1. Using ```read()```:-> The read() method from std::fs::File reads the entire contents of a file as a Vec<u8>. This is useful when you want to handle the file contents as a byte vector.
    ```read_to_end```: This reads the entire file content into a buffer (```Vec<u8>```). This is suitable when you need to process the raw bytes of the file.
2. ```read_string()```: -> The ```read_to_string()``` method reads the entire file as a String. It is suitable when the file contains textual data.
    ```read_to_string```: This reads the entire file content as a String. This is great for text files as it handles encoding automatically.

3. ```BufReader``` ->For large files or when you want to read the file line by line, using a BufReader is more efficient. It allows you to read the file in chunks (buffered) and process it line by line.
```BufReader```: This wraps the file to read it in a buffered manner, reducing the number of system calls when reading.
```lines()```: Returns an iterator that allows you to read the file line by line.


Write:
```write_all()```
The ```write_all()``` method writes the entire content to a file. It is typically used when you want to overwrite the content of the file (or create a new file if it doesn't exist).

Append to file=>
Use ```OpenOptions``` enum
If you want to append data to an existing file without overwriting it, you can use the ```OpenOptions``` struct. This struct provides more granular control over how files are opened (e.g., ```read, write, append```).

```OpenOptions```: This is a builder pattern that allows you to specify various options for opening a file.
```
.write(true): Allows writing to the file.
.create(true): Creates the file if it does not exist.
.append(true): Opens the file in append mode, so data is added at the end of the file without overwriting it.
.open(file_path): Actually opens the file with the specified options.

.read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("foo.txt");


```
#### Working with JSON file

**Reading JSON from a File**

To read JSON data from a file, you’ll need to deserialize it into a Rust struct. The ```serde_json::from_str``` function does this deserialization.
To deserialize it into a Rust struct.

Define a Rust Struct
Define a struct that matches the structure of your JSON

```#[derive(Deserialize)]```: Automatically implements the Deserialize trait for struct, allowing it to be deserialized from JSON.

```#[derive(Serialize)]```: Automatically implements the Serialize trait, allowing it to be serialized to JSON.

```serde_json::from_str```: This function takes a string and attempts to deserialize it into the specified Rust type (Person in this case).

```read_to_string```: Reads the contents of the file into a string.

**Writing JSON to a File**

To write JSON data to a file, you need to serialize the Rust struct into a JSON string using ```serde_json::to_string``` (or to_writer to write directly to a file).


- ```serde and serde_json``` are the most common crates for working with JSON in Rust.

- You can deserialize JSON into Rust structs using ```serde_json::from_str```.
- You can serialize Rust structs into JSON using ```serde_json::to_string or serde_json::to_writer```.
- Nested JSON data can be mapped to nested Rust structs to represent complex data structures.