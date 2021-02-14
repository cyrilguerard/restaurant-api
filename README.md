# restaurant-api

An API to manage client orders per table in a great restaurant.

# build

```sh
git clone https://github.com/cyrilguerard/restaurant-api.git
cd restaurant-api
./build.sh
```

# run
```sh
./run.sh
```

Or

```sh
docker build -t restaurant-api .
docker run -d --rm -p 8000:8000 --name restaurant-api restaurant-api
```

# test
```sh
./test.sh
```

TODO: add unit tests on service layer

# documentation

TODO: document inside the code (module, function)
TODO: document API using swagger

Server URL: http://localhost:8000/api/v1

| Method | Resource                             | Description                      | Request Body | Response Body                                                                                                                                  | Errors                                                         |
|--------|--------------------------------------|----------------------------------|--------------|------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------|
| GET    | /menu-items                          | List all the menu items          | None         | [   {      "id": u16,      "name": string    },   ... ]                                                                                        |                                                                |
| GET    | /tables/<table_id>/orders            | List the orders for the table    | None         | [   {     "id": u16,     "item": {       "id": u16,       "name": string     },     "ready_at": datetime(YYYY-MM-DDTHH:MM:SS.SSS)   },   ... ] | 404: Bad Request  {   "reason": string,    "message": string } |
| GET    | /tables/<table_id>/orders/<order_id> | Get one order for the table      | None         | {    "id": u16,   "item": {     "id": u16,     "name": string   },   "ready_at": datetime(YYYY-MM-DDTHH:MM:SS.SSS) }                           | 404: Bad Request  {   "reason": string,    "message": string } |
| POST   | /tables/<table_id>/orders?<item_id>  | Order a menu item for the table  | None         | {    "id": u16,   "item": {     "id": u16,     "name": string   },   "ready_at": datetime(YYYY-MM-DDTHH:MM:SS.SSS) }                           | 404: Bad Request  {   "reason": string,    "message": string } |
| DELETE | /tables/<table_id>/orders/<order_id> | Cancel a menu item for the table | None         | None                                                                                                                                           | 404: Bad Request  {   "reason": string,    "message": string } |
