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

### Solution

#### Extras

Some special channels have a function that returns an `Option<Extra>` with the added data. I need to check which extra
data news/store has over text and see if I can just merge them with a kind field.

#### Enum

- Guild
  - Text (kind field)
  - Voice (extra field)
  - Thread
  - Category
- Private
  - DirectMessage
  - Group
