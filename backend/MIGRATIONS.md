# Database Migrations with SQLX

This project uses SQLX for database migrations. Migrations are automatically run when the application starts.

## Migration Structure

Migrations are stored in the `migrations/` directory of each service:
- `auth-service/migrations/`
- `settings-service/migrations/`

Each migration consists of two files:
- `{timestamp}_{description}.up.sql` - SQL to apply the migration
- `{timestamp}_{description}.down.sql` - SQL to revert the migration

## Running Migrations

### Automatic (Recommended)

Migrations run automatically when you start the application. The application will:
1. Connect to the database
2. Check which migrations have been applied
3. Run any pending migrations
4. Print a success message

You'll see this output when migrations run:
```
✅ Database migrations completed successfully
🚀 Server started successfully
```

### Manual

You can also run migrations manually using the SQLX CLI:

```bash
# Install SQLX CLI (if not already installed)
cargo install sqlx-cli --no-default-features --features postgres

# Run pending migrations
cd backend/auth-service
sqlx migrate run

cd backend/settings-service
sqlx migrate run
```

## Creating New Migrations

To create a new migration:

```bash
# Navigate to the service directory
cd backend/auth-service  # or backend/settings-service

# Create a new migration
sqlx migrate add <description>

# This creates two files:
# - migrations/{timestamp}_{description}.up.sql
# - migrations/{timestamp}_{description}.down.sql
```

Edit the generated files to add your SQL:

**{timestamp}_{description}.up.sql** - Add your schema changes:
```sql
CREATE TABLE example (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL
);
```

**{timestamp}_{description}.down.sql** - Add the reverse operation:
```sql
DROP TABLE example;
```

## Migration History

SQLX tracks applied migrations in the `_sqlx_migrations` table. This table is automatically created and managed by SQLX.

## Rolling Back Migrations

To revert the last migration:

```bash
sqlx migrate revert
```

To revert multiple migrations:
```bash
sqlx migrate revert
sqlx migrate revert
# ... repeat as needed
```

## Migration Best Practices

1. **Always include down migrations** - Make sure every migration can be reversed
2. **Test migrations** - Test both up and down migrations before committing
3. **Use transactions** - Wrap complex migrations in BEGIN/COMMIT for atomicity
4. **Avoid breaking changes** - When possible, make changes backward-compatible
5. **Document complex migrations** - Add comments explaining non-obvious changes

## Converted from Diesel

This project was originally using Diesel ORM. The migrations have been converted to SQLX format:
- Diesel migrations were in `migrations/{timestamp}_{description}/` with `up.sql` and `down.sql`
- SQLX migrations are flat files: `migrations/{timestamp}_{description}.{up,down}.sql`

All existing migrations have been preserved and work identically with SQLX.
