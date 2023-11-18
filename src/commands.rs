use crate::{
    Context,
    ApplicationError
};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), ApplicationError> {

    poise::send_reply(ctx, |rep| rep
        .embed(|e| e
            .title("Pong")
            .color((230, 126, 34))
            .timestamp("2017-01-03T23:00:00Z")
        )
        .ephemeral(true)
    ).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn debug(ctx: Context<'_>) -> Result<(), ApplicationError> {
    ctx.reply(format!("your id: {}", ctx.author().id)).await?;
    Ok(())
}
