   #[derive(Debug)]
   struct Server {
      name: String,
      location: (f64, f64),
      ip: (i8, i8, i8, i8)
   }

   let server: Server = Server {
      name: String::from("USA"),
      location: (124.02, 21.5),
      ip: (127, 0, 0, 1)
   };

   trait ServerKit {
      fn init(&self) -> Server;
   }

   impl ServerKit for Server {
      fn init(&self) -> Server {
         Server {
            name: String::from("New Server"),
            location: (0.00, 0.00),
            ip: (127, 0, 0, 1)
         }
      }
   }


   println!("{:?}", server);
   println!("{:?}", server.init());