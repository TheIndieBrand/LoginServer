# LoginServer

Servidor de login centralizado para los servicios de The Indie Brand. Corre en `login.theindiebrand.es`; las demás webs redirigen aquí y la autenticación se hace con Discord (OAuth2).

Stack: Rust, Axum, SQLite (`rusqlite`), JWT (`jsonwebtoken`).

## Cómo funciona

1. Una web redirige al usuario a `https://login.theindiebrand.es/login?redirect=<url>`.
2. El servidor valida `redirect` (solo `theindiebrand.es` y sus subdominios) y manda al usuario a Discord.
3. Discord vuelve a `/auth/discord/callback`. Se comprueba el `state`, se canjea el código y se guarda el usuario (id, nombre, avatar).
4. Se crea la cookie `session` (JWT, 7 días, `HttpOnly`, `SameSite=Lax`) en el dominio `.theindiebrand.es` y se devuelve al usuario a la web de origen.

## Rutas

| Ruta | Descripción |
|---|---|
| `GET /login?redirect=URL` | Inicia el login con Discord |
| `GET /auth/discord/callback` | Callback de Discord (no usar directamente) |
| `GET /me` | `{id, name, avatar_url}` o `401`. Lee la cookie `session` o `Authorization: Bearer <jwt>` |
| `GET /logout?redirect=URL` | Borra la sesión y redirige |
| `GET /health` | Devuelve `ok` |

## Estructura

```
sql/
  schema.sql          Esquema de la base de datos
  upsert_user.sql     Alta/actualización de usuario
src/
  main.rs             Arranque y rutas
  config.rs           Variables de entorno
  state.rs            Estado compartido
  db.rs               Gestor de la base de datos (carga los .sql)
  discord.rs          Llamadas a la API de Discord
  jwt.rs              Firma y verificación de sesiones
  cookies.rs          Construcción de cookies
  redirect.rs         Validación de redirecciones
  error.rs, util.rs   Errores y utilidades
  handlers/           Un archivo por ruta (login, callback, me, logout, health)
```

Las consultas SQL se incluyen en el binario al compilar (`include_str!`), así que no hace falta desplegar la carpeta `sql/`.

## Configuración

1. En el [Discord Developer Portal](https://discord.com/developers/applications) crea una aplicación.
2. En **OAuth2 → Redirects** añade `https://login.theindiebrand.es/auth/discord/callback`.
3. Copia `.env.example` a `.env` y rellena:

| Variable | Descripción |
|---|---|
| `DISCORD_CLIENT_ID` / `DISCORD_CLIENT_SECRET` | Credenciales de la aplicación de Discord |
| `DISCORD_REDIRECT_URI` | Debe coincidir exactamente con la registrada en Discord |
| `JWT_SECRET` | Mínimo 32 caracteres aleatorios |
| `COOKIE_DOMAIN` | `.theindiebrand.es` en producción; vacío en local |
| `COOKIE_SECURE` | `true` en producción (HTTPS); `false` en local |
| `ALLOWED_REDIRECT_SUFFIX` | `theindiebrand.es` |
| `BIND_ADDR` | Por defecto `127.0.0.1:3000` |
| `DATABASE_PATH` | Por defecto `login.db` |

Generar un `JWT_SECRET`:

```powershell
-join ((1..48) | ForEach-Object { '{0:x}' -f (Get-Random -Maximum 16) })
```

## Desarrollo local

Para probar en local, usa en `.env`:

```
DISCORD_REDIRECT_URI=http://localhost:3000/auth/discord/callback
COOKIE_DOMAIN=
COOKIE_SECURE=false
```

Añade esa misma redirect URI en Discord y ejecuta:

```bash
cargo run
```

Con `COOKIE_SECURE=false` se permiten redirecciones a `localhost` y `127.0.0.1`.

## Despliegue

Compila el binario optimizado (usa poca RAM, ~10-20 MB en ejecución):

```bash
cargo build --release
```

El ejecutable queda en `target/release/login-server` (`.exe` en Windows). Si el VPS es pequeño, compila en local o en CI y sube solo el binario.

Requisitos:

- Un proxy inverso con HTTPS (Caddy, nginx…) delante, apuntando a `BIND_ADDR`.
- Un registro DNS `login.theindiebrand.es` hacia el servidor.
- El `.env` junto al binario, o las variables definidas en el servicio de systemd.

Ejemplo con Caddy:

```
login.theindiebrand.es {
    reverse_proxy 127.0.0.1:3000
}
```

Ejemplo de servicio systemd (`/etc/systemd/system/login-server.service`):

```ini
[Unit]
Description=theindiebrand login server
After=network.target

[Service]
WorkingDirectory=/opt/login-server
ExecStart=/opt/login-server/login-server
EnvironmentFile=/opt/login-server/.env
Restart=on-failure
User=loginserver

[Install]
WantedBy=multi-user.target
```

## Integrar otra web

**Iniciar sesión:** enlaza a

```
https://login.theindiebrand.es/login?redirect=https://miweb.theindiebrand.es/
```

**Saber quién es el usuario:** como la cookie `session` está en `.theindiebrand.es`, el backend de la web la recibe. Reenvíala a `/me`:

```ts
const res = await fetch("https://login.theindiebrand.es/me", {
  headers: { cookie: `session=${sessionCookie}` },
});
if (res.status === 401) { /* no autenticado */ }
const user = await res.json(); // { id, name, avatar_url }
```

También se puede enviar el JWT como `Authorization: Bearer <jwt>`.

**Cerrar sesión:** enlaza a `https://login.theindiebrand.es/logout?redirect=https://miweb.theindiebrand.es/`.

## Seguridad

- Las redirecciones solo se aceptan hacia `ALLOWED_REDIRECT_SUFFIX` y sus subdominios (rechaza `theindiebrand.es.evil.com`, credenciales en la URL, etc.).
- El parámetro `state` de OAuth se guarda en una cookie `HttpOnly` y se comprueba en el callback.
- No compartas `JWT_SECRET` con otras webs; verifican la sesión con `/me`.
- Si una web usa un dominio distinto de `theindiebrand.es`, la cookie compartida no le llega y haría falta un intercambio de token.
- Nunca subas `.env` al repositorio (ya está en `.gitignore`).
