db.createUser({
    user: "rustUser",
    pwd: "rustPass",
    roles: [{
        role: "readWrite",
        db: "rust"
    }]
});
