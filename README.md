# Rust Firebase

Rust based library for interacting with the Firebase REST API.

## Full Documentation

The APIs in this README do not document all of the APIs available!
Go to the official docs for the most up to date version of the API.

## Load the crate!

Don't forget to include the library in your project:
```Rust
extern crate firebase;
use firebase::Firebase;
```

## Creating a Firebase reference

### Simple
You can currently create a simple reference to your firebase server:

```Rust
let firebase = Firebase::new("https://<your-firebase>.firebaseio.com");
```

### Authenticated
Or you can create an authenticated connection by supplying your [auth](https://www.firebase.com/docs/rest/guide/user-auth.html) token:

```Rust
let firebase = Firebase::authed("https://<your-firebase>.firebaseio.com", "<token>");
```

**NOTE:** You must send your requests through HTTPS or Firebase will reject it.
Not specifying HTTPS will also result in an error: ParseError::UrlIsNotHTTPS

## Walking the database

Reference nested objects in your server like so:

```Rust
let show = firebase.at("/shows/futurama"); // points to /shows/futurama
let episode = show.at("s10/meanwhile");    // points to /shows/futurama/s10/meanwhile
```

Slashes and .json extensions will be handled accordingly:

```Rust
// All of the following are equivalent:
let show = firebase.at("/shows/futurama.json");
let show = firebase.at("shows/futurama/");
let show = firebase.at("/shows/futurama/");
```

## Working with data

### Reading data

Reading data can be done with a simple call to ```.get()```
```Rust
let response = show.get();
```

### Writing data

```Rust
let description = episode.at("description");
let response = description.set(serde_json::json!("the last episode"));
```

### Pushing data

```Rust
let episodes = firebase.at("/shows/futurama/episodes");
let response = episodes.push(serde_json::json!("The Lost Episode!"));
```

### Updating data

```Rust
let description = episode.at("description");
let response = description.update(serde_json::json!("the penultimate episode"));
```

### Removing data

```Rust
let episode = firebase.at("/shows/futurama/s10/meanwhile");
let response = episode.remove();
```

## Requests with parameters

```Rust
let episodes = firebase.at("/shows/futurama/episodes");
let top5 = episodes.order_by("\"imdb\"").limit_to_first(5).get();
```

The full list of supported parameters are listed here:

 - ```order_by```
 - ```limit_to_first```
 - ```limit_to_last```
 - ```start_at```
 - ```end_at```
 - ```equal_to```
 - ```shallow```
### Working with JSON values

* Json is received and sent as a serde_json Value object.

Example
```Rust
let json = serde_json::json!({ "name": "David Smith" });

let people = firebase.at("/earth/us/indiana");
let response = people.push(json);

...


#[derive(Serialize,Deserialize)]
struct Person {
    name: String
}
...
let person = Person {
    name: "Gavin"
};

let json = serde_json::to_value(person);
let people = firebase.at("/earth/us/utah");
let response = people.push(json);
```
