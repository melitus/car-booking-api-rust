# Car Booking API Rust🛡️

# Tools

- Rust
- Actix Web
- Diesel
- PostgreSQL

## Getting Started

The easiest way to get started is to clone the repository:

# clone the repository

```
git clone https://github.com/melitus/car-booking-api-rust.git
```

# Change directory

```
cd car-booking-api-rust
```

# Install Postgres with docker-compose

```
sudo docker-compose up

```
# TO ssh into docker 
`sudo docker exec -it postgres bash`
Inside the shell, enter interact with the postgress `psql -h localhost -p 5432 -U db_user -d car_booking_db`


# start the server

```
cargo watch -x run
```

Note: It is recommended to install cargo-watch for livereloading - It watches for any changes in your rust app and automatically restarts the server

# Deployment
### Deployment to Heroku

<img src="https://upload.wikimedia.org/wikipedia/en/a/a9/Heroku_logo.png" width="200">

- Download and install [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli#download-and-install)
- In a terminal, run `heroku login` and enter your Heroku credentials
- From *your app* directory run `heroku create`
- Use the command `heroku config:set KEY=val` to set the different environment variables (KEY=val) for your application (i.e.  `heroku config:set BASE_URL=[heroku App Name].herokuapp.com` etc.)

- Do `git add .`
- Do `git commit -m" reason for commit"`
- Lastly, do `git push heroku master`.

Please note that you may also use the [Herko Dashboard](https://dashboard.heroku.com) to set or modify the configurations for your application.

# View live demo and test with any api
I used postman to test the api.
To use postman, go to the project doc folder and import the docs file into your postman client to ease the testing

## User Endpoint
 - Login - `https://car-booking-backend.herokuapp.com/v1/api/users/login`
 - Register - `https://car-booking-backend.herokuapp.com/v1/api/users/register`

## Car Endpoints

- Get all cars - `https://car-booking-backend.herokuapp.com/v1/api/cars?pagesize=10&pagenumber=1`

- Book a car - `https://car-booking-backend.herokuapp.com/v1/api/carrs`

- Get previous booking filtered by user - `https://car-booking-backend.herokuapp.com/v1/api/cars/24d551b9-6845-443a-ba9e-de18dd0b4fec/previous?pagesize=10&pagenumber=1`



