use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tic_lib::models::gamestate::GameState;

struct Client {
    socket: TcpStream,
}

impl Client {
    pub fn write(&mut self) {
        let counter: u32 = 0;
        self.socket.write(counter.to_string().as_bytes()).unwrap();
    }
}

struct ClientThread {
    inner: Client,
}

impl ClientThread {
    pub fn client_loop(&mut self) {
        let client = &mut self.inner;

        client.write();
    }
}

struct Server {
    pub clients: Vec<Client>,
    pub state: GameState,
}

impl Server {
    fn new() -> Server {
        Server {
            clients: vec![],
            state: GameState {
                turns: 0,
                players: Vec::new(),
                board: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                current_player: None,
            },
        }
    }

    fn _reset_game_state(&mut self) {
        self.state = GameState {
            turns: 0,
            players: Vec::new(),
            board: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            current_player: None,
        };
    }

    fn add_client(&mut self, s: TcpStream) {
        let client = Client {
            socket: s.try_clone().unwrap(),
        };

        println!("New client: {}", client.socket.peer_addr().unwrap());

        let mut client_thread = ClientThread { inner: client };

        thread::spawn(move || loop {
            client_thread.client_loop();
        });
    }

    pub fn server_loop(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:3333").unwrap();

        loop {
            match listener.accept() {
                Ok((socket, _addr)) => {
                    self.add_client(socket);
                }
                Err(e) => println!("Couldn't get client: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.server_loop();
}
