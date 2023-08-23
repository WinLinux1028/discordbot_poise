use poise::serenity_prelude::{self as serenity, Mentionable};

pub async fn process<'a>(
    ctx: &'a serenity::Context,
    message: &serenity::Message,
    color: u32,
    action: &'a str,
) -> Option<serenity::CreateMessage<'a>> {
    let channel_name = match message.channel_id.name(ctx).await {
        Some(s) => s,
        None => return None,
    };

    // embedの雛形を作る
    let mut embed = serenity::CreateEmbed::default();
    embed.color(color);

    let name = format!("{}がメッセージを{}しました", message.author.tag(), action);
    embed.author(|author| author.name(name).icon_url(message.author.face()));

    let timestamp_local = message.timestamp.with_timezone(&chrono::offset::Local);
    let timestamp = timestamp_local.format("%Y年%m月%d日%H時%M分%S秒%3f %Z");
    let channel = format!("#{}({})", channel_name, message.channel_id.mention());
    let footer = format!(
        "メッセージの送信日: {}\n{}\nメッセージID: {}",
        timestamp, channel, message.id
    );
    embed.field("詳細情報", footer, false);

    // 画像を複数のembedにする
    let mut embeds = Vec::new();
    let images = message
        .attachments
        .iter()
        .filter(|file| file.width.is_some() && file.height.is_some());
    for image in images {
        let mut embed = embed.clone();
        embed.image(&image.proxy_url);
        embeds.push(embed);
    }

    // 画像がない場合対策
    if embeds.is_empty() {
        embeds.push(embed);
    }
    // 最初のembedにはメッセージの本文を入れる
    embeds[0].description(&message.content);

    let mut message = serenity::CreateMessage::default();
    message.set_embeds(embeds);

    Some(message)
}
