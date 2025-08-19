# oai-usage

Simple cli tool that fetches and displays OpenAI API usage data (for completions) for the last N days.
Written by claude and gpt-5. PRs welcome.

```
Fetching OpenAI usage data for the past 3 days...
+------------+------------------+----------+--------------+--------------+---------------+----------+
| Date       | Model            | Requests | Input Tokens | Cached Input | Output Tokens | Cost ($) |
+===================================================================================================+
| 2025-08-17 | gpt-5-2025-08-07 |      134 |      467,885 |    9,734,144 |        53,800 |   2.3396 |
|------------+------------------+----------+--------------+--------------+---------------+----------|
| 2025-08-18 | gpt-5-2025-08-07 |      111 |      120,039 |    5,645,568 |        38,326 |   1.2390 |
|------------+------------------+----------+--------------+--------------+---------------+----------|
| 2025-08-19 | gpt-5-2025-08-07 |       38 |       52,319 |      646,272 |        20,614 |   0.3523 |
|------------+------------------+----------+--------------+--------------+---------------+----------|
| TOTAL      |                  |      283 |      640,243 |   16,025,984 |       112,740 |   3.9310 |
+------------+------------------+----------+--------------+--------------+---------------+----------+
```

Use with the `watch` command to see the pennies flow in real time, eg `watch -c -n 5 -d oai-usage`.

## Install
Install rust. https://www.rust-lang.org/tools/install

`cargo install oai-usage`

## Usage
- Set `OPENAI_ADMIN_KEY` in your environment. Note that this needs to be an "admin key", which is distinct from the API key you'd use for calling a model. You can create an admin key in the openai settings (not the Dashboard). We recommend creating an admin key with read-only permissions to the usage api, and no other permissions.
- Run: `oai-usage [days]` (default: `3`).

Example: `OPENAI_ADMIN_KEY=... oai-usage 7`

Note: The pricing table is embedded and may not always match the most recent public prices.

## License
Dual-licensed under MIT or Apache-2.0 at your option.

- MIT: http://opensource.org/licenses/MIT
- Apache-2.0: http://www.apache.org/licenses/LICENSE-2.0
