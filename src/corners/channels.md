# Corner case - Channels

How should we handle channels? Let's look at other libraries!

### [Serenity](https://docs.rs/serenity/0.10.10/serenity/model/channel/enum.Channel.html)

Serenity has an enum of groups of channel types which have different structs in them:
- Guild - Text, voice, etc
- Private
- Category

### [Twilight](https://docs.rs/twilight-model/0.9.0/twilight_model/channel/enum.Channel.html)

Twilight has a similar system, but with different grouping and the guild channels are an enum:
- Group
- Guild
  - Category
  - NewsThread
  - PrivateThread
  - PublicThread
  - Text
  - Voice
  - Stage
- Private


### Choices

this kinda enum:

Channel
- Guild
  - Text
    - Normal
    - Store
    - News
  - Voice
    - Stage
    - NotStage
  - Thread
  - Category
- Private
  - DirectMessage
  - Group

or like

Channel
- Everything
