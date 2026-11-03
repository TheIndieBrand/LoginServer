INSERT INTO users (id, username, avatar, created_at, last_login)
    VALUES (?1, ?2, ?3, ?4, ?4)
    ON CONFLICT(id)
        DO UPDATE SET username = ?2, avatar = ?3, last_login = ?4
