# ☣️ Build Ayoub Servers

* ☢️ To run the `event` server just change the `CURRENT_SERVICE` variable to `event` value.

* ☢️ To run the `auth` server just change the `CURRENT_SERVICE` variable to `auth` value.

* ☢️ To run the `game` server just change the `CURRENT_SERVICE` variable to `game` value.

> Behalf of the user the server will assign the role_id and side_id for the user

```console
$ cargo build --bin ayoub --release
```

# 🧿 God Access

* Create the deck

* Create the event

* Attach the deck id for the created event

* Can update the role, side, ability for a player and phases for the event

* Can chain two players together

# 🎎 Player Access

* Reserve and pay for the event and his/her role

# 📌 TODOs

* **Zarinpal API token**

* **OTP API token**

* send OTP response API from career issue with serde parser 

* **postman collection (status constants + unix timestamps)** 

* auth guard and access level on APIs

* server signal handling 

* add client algorithm

