This repo is a work in progress example of making complex api with diesel, juniper, actix, juniper-from-schema and juniper-eager-loading (needed to solve N+1 issues and this lib work with juniper-from-schema).  
The goal is also to find where we can ease some process between diesel and juniper (matching types...).  
I am pretty new to rust so I encourage everybody to check this code and sugest improvements !

## Init
Just run `diesel database setup --database-url postgres://postgres:postgres@localhost/graphqlapi`

## Run
```bash
docker-compose up -d
cargo run
```

## Schemes

### [GraphQL](src/graphql/schema.graphql)

### Database
![Database scheme](./test_data_model_v1.png)

## Observations
- When using enum we need to implement lot of code to be able to store it into diesel (create new enum, impl `From` for the diesel enum and for the graphql enum) -> [see this file](./src/models/mod.rs) 


## TODO
- [ ] Add tests
- [ ] Add metrics route (prometheus for exemple)
- [ ] Add health route (for kube check health)
- [ ] Create public and private api with differents ports but sharing database, metrics...
