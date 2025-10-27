# Separate Databases Configuration

## Overview

Each service in the Brewget backend has its own dedicated PostgreSQL database:

- **auth-service**: `brewget_auth` - Contains user authentication and authorization data
- **settings-service**: `brewget_settings` - Contains user settings and preferences

This separation provides several benefits:
- **Isolation**: Database failures in one service don't affect others
- **Security**: Each service only has access to its own data
- **Scalability**: Databases can be scaled independently based on service needs
- **Maintenance**: Easier to backup, restore, and manage individual service data

## Database Schema

### auth-service (`brewget_auth`)
Tables:
- `users` - User accounts and authentication data
- `tokens` - JWT tokens for session management
- `activation_links` - Email verification links
- `forgot_password_links` - Password reset links
- `_sqlx_migrations` - Migration tracking

### settings-service (`brewget_settings`)
Tables:
- `user_settings` - User preferences and configuration
- `_sqlx_migrations` - Migration tracking

## Environment Variables

### auth-service
- `PG_URL` - PostgreSQL server hostname/IP
- `PG_USERNAME` - Database username
- `PG_PASSWORD` - Database password
- `AUTH_PG_DATABASE` - Auth service database name (default: falls back to `PG_DATABASE`)

### settings-service
- `PG_URL` - PostgreSQL server hostname/IP
- `PG_USERNAME` - Database username
- `PG_PASSWORD` - Database password
- `SETTINGS_PG_DATABASE` - Settings service database name (default: falls back to `PG_DATABASE`)

## Backwards Compatibility

For backwards compatibility, both services will fall back to the `PG_DATABASE` environment variable if their service-specific variable is not set:

- If `AUTH_PG_DATABASE` is not set, auth-service uses `PG_DATABASE`
- If `SETTINGS_PG_DATABASE` is not set, settings-service uses `PG_DATABASE`

This allows for gradual migration from a shared database to separate databases.

## Setting Up Separate Databases

### Docker Compose (Production)

The `docker-compose.yaml` is already configured to use separate databases:

```yaml
auth-service:
  environment:
    - AUTH_PG_DATABASE=brewget_auth

settings-service:
  environment:
    - SETTINGS_PG_DATABASE=brewget_settings
```

### Local Development

Create both databases in your PostgreSQL server:

```sql
CREATE DATABASE brewget_auth;
CREATE DATABASE brewget_settings;
```

Set environment variables for each service:

**For auth-service:**
```bash
export PG_URL=localhost
export PG_USERNAME=postgres
export PG_PASSWORD=yourpassword
export AUTH_PG_DATABASE=brewget_auth
```

**For settings-service:**
```bash
export PG_URL=localhost
export PG_USERNAME=postgres
export PG_PASSWORD=yourpassword
export SETTINGS_PG_DATABASE=brewget_settings
```

### Migrations

Migrations run automatically when each service starts. Each service will:

1. Connect to its own database
2. Check for pending migrations in its `migrations/` directory
3. Apply any unapplied migrations
4. Track migration history in its own `_sqlx_migrations` table

You'll see output like:
```
✅ Database migrations completed successfully
🚀 Server started successfully
```

### Manual Migration Management

Using the SQLX CLI for each service:

**auth-service:**
```bash
cd backend/auth-service
DATABASE_URL="postgres://username:password@localhost/brewget_auth" sqlx migrate run
```

**settings-service:**
```bash
cd backend/settings-service
DATABASE_URL="postgres://username:password@localhost/brewget_settings" sqlx migrate run
```

## Troubleshooting

### Database Not Found

If you see an error about the database not existing:

```
database "brewget_auth" does not exist
```

Create the database manually:
```sql
CREATE DATABASE brewget_auth;
```

### Connection Issues

Ensure:
1. PostgreSQL server is running
2. Credentials are correct
3. The user has permissions to access the database
4. Network connectivity allows connections

### Migration Failures

If migrations fail:
1. Check the database logs
2. Verify the migration SQL syntax
3. Ensure previous migrations completed successfully
4. Check the `_sqlx_migrations` table for migration status

## Migration from Shared Database

If you're migrating from a shared database setup:

1. **Backup the existing database**
   ```bash
   pg_dump brewget > brewget_backup.sql
   ```

2. **Create new databases**
   ```sql
   CREATE DATABASE brewget_auth;
   CREATE DATABASE brewget_settings;
   ```

3. **Migrate auth-service data**
   ```sql
   \c brewget_auth
   -- Copy relevant tables from backup
   ```

4. **Migrate settings-service data**
   ```sql
   \c brewget_settings
   -- Copy relevant tables from backup
   ```

5. **Update environment variables**
   - Set `AUTH_PG_DATABASE=brewget_auth`
   - Set `SETTINGS_PG_DATABASE=brewget_settings`

6. **Restart services**
   - Migrations will run automatically on the new databases
   - Verify data integrity

## Security Considerations

- Each service should use separate database users with minimal privileges
- Grant only necessary permissions to each service's database user
- Regularly backup each database independently
- Monitor each database for unusual activity

Example permissions:

```sql
-- Auth service user
CREATE USER auth_service_user WITH PASSWORD 'auth_password';
GRANT CONNECT ON DATABASE brewget_auth TO auth_service_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO auth_service_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO auth_service_user;

-- Settings service user
CREATE USER settings_service_user WITH PASSWORD 'settings_password';
GRANT CONNECT ON DATABASE brewget_settings TO settings_service_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO settings_service_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO settings_service_user;
```
