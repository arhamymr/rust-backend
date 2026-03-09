use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::EmailVerified).boolean().not_null())
                    .col(ColumnDef::new(User::Image).string().null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::Role).string().null())
                    .col(ColumnDef::new(User::Banned).boolean().null())
                    .col(ColumnDef::new(User::BanReason).string().null())
                    .col(ColumnDef::new(User::BanExpires).timestamp().null())
                    .to_owned(),
            )
            .await?;

        // Create account table
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Account::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Account::AccountId).string().not_null())
                    .col(ColumnDef::new(Account::ProviderId).string().not_null())
                    .col(ColumnDef::new(Account::UserId).string().not_null())
                    .col(ColumnDef::new(Account::AccessToken).string().null())
                    .col(ColumnDef::new(Account::RefreshToken).string().null())
                    .col(ColumnDef::new(Account::IdToken).string().null())
                    .col(ColumnDef::new(Account::AccessTokenExpiresAt).timestamp().null())
                    .col(ColumnDef::new(Account::RefreshTokenExpiresAt).timestamp().null())
                    .col(ColumnDef::new(Account::Scope).string().null())
                    .col(ColumnDef::new(Account::Password).string().null())
                    .col(ColumnDef::new(Account::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Account::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_user_id_user_id")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create session table
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Session::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(Session::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::IpAddress).string().null())
                    .col(ColumnDef::new(Session::UserAgent).string().null())
                    .col(ColumnDef::new(Session::UserId).string().not_null())
                    .col(ColumnDef::new(Session::ImpersonatedBy).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_user_id_user_id")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create verification table
        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Verification::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Verification::Identifier).string().not_null())
                    .col(ColumnDef::new(Verification::Value).string().not_null())
                    .col(ColumnDef::new(Verification::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Verification::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Verification::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop in reverse order of creation to satisfy FK constraints
        manager
            .drop_table(Table::drop().table(Verification::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    EmailVerified,
    Image,
    CreatedAt,
    UpdatedAt,
    Role,
    Banned,
    BanReason,
    BanExpires,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    AccountId,
    ProviderId,
    UserId,
    AccessToken,
    RefreshToken,
    IdToken,
    AccessTokenExpiresAt,
    RefreshTokenExpiresAt,
    Scope,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    ExpiresAt,
    Token,
    CreatedAt,
    UpdatedAt,
    IpAddress,
    UserAgent,
    UserId,
    ImpersonatedBy,
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    Id,
    Identifier,
    Value,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}
