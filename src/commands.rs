use crate::{
    Context,
    ApplicationError
};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), ApplicationError> {

    poise::send_reply(ctx, |f| f
        .embed(|e| e
            .title("Pong.")
            .color((230, 126, 34))
        )
        .ephemeral(true)
    ).await?;
    Ok(())
}
