## Example

curl -s https://api.openai.com/v1/organization/usage/completions \
   -H "Authorization: Bearer $OPENAI_ADMIN_KEY" \
   -G \
   --data-urlencode "start_time=1754550000" \
   --data-urlencode "group_by=model"

Result:

```json
{
  "object": "page",
  "data": [
    {
      "object": "bucket",
      "start_time": 1754956800,
      "end_time": 1755043200,
      "results": [
        {
          "object": "organization.usage.completions.result",
          "input_tokens": 14350791,
          "output_tokens": 96969,
          "num_model_requests": 270,
          "project_id": null,
          "user_id": null,
          "api_key_id": null,
          "model": "gpt-5-2025-08-07",
          "batch": null,
          "input_cached_tokens": 12464896,
          "input_audio_tokens": 0,
          "output_audio_tokens": 0
        }
      ]
    },
    {
      "object": "bucket",
      "start_time": 1755043200,
      "end_time": 1755129600,
      "results": [
        {
          "object": "organization.usage.completions.result",
          "input_tokens": 18449801,
          "output_tokens": 138957,
          "num_model_requests": 364,
          "project_id": null,
          "user_id": null,
          "api_key_id": null,
          "model": "gpt-5-2025-08-07",
          "batch": null,
          "input_cached_tokens": 17526528,
          "input_audio_tokens": 0,
          "output_audio_tokens": 0
        },
        {
          "object": "organization.usage.completions.result",
          "input_tokens": 1233391,
          "output_tokens": 7754,
          "num_model_requests": 29,
          "project_id": null,
          "user_id": null,
          "api_key_id": null,
          "model": "gpt-5-mini-2025-08-07",
          "batch": null,
          "input_cached_tokens": 1091584,
          "input_audio_tokens": 0,
          "output_audio_tokens": 0
        }
      ]
    },
    {
      "object": "bucket",
      "start_time": 1755129600,
      "end_time": 1755216000,
      "results": [
        {
          "object": "organization.usage.completions.result",
          "input_tokens": 13718197,
          "output_tokens": 121397,
          "num_model_requests": 321,
          "project_id": null,
          "user_id": null,
          "api_key_id": null,
          "model": "gpt-5-2025-08-07",
          "batch": null,
          "input_cached_tokens": 12887680,
          "input_audio_tokens": 0,
          "output_audio_tokens": 0
        }
      ]
    }
  ],
  "has_more": true,
  "next_page": "page_AAAAAGijgQLgv6bHAAAAAGihG4A="
}
```


## Query parameters

| Parameter | Type | Required/Optional | Description |
|-----------|------|-------------------|-------------|
| start_time | integer | Required | Start time (Unix seconds) of the query time range, inclusive. |
| api_key_ids | array | Optional | Return only usage for these API keys. |
| batch | boolean | Optional | If true, return batch jobs only. If false, return non-batch jobs only. By default, return both. |
| bucket_width | string | Optional | Defaults to 1d. Width of each time bucket in response. Currently 1m, 1h and 1d are supported, default to 1d. |
| end_time | integer | Optional | End time (Unix seconds) of the query time range, exclusive. |
| group_by | array | Optional | Group the usage data by the specified fields. Support fields include project_id, user_id, api_key_id, model, batch or any combination of them. |
| limit | integer | Optional | Specifies the number of buckets to return.<br><br>bucket_width=1d: default: 7, max: 31<br>bucket_width=1h: default: 24, max: 168<br>bucket_width=1m: default: 60, max: 1440 |
| models | array | Optional | Return only usage for these models. |
| page | string | Optional | A cursor for use in pagination. Corresponding to the next_page field from the previous response. |
| project_ids | array | Optional | Return only usage for these projects. |
| user_ids | array | Optional | Return only usage for these users. |


## Completions usage object
The aggregated completions usage details of the specific time bucket.

| Field | Type | Description |
|-------|------|-------------|
| api_key_id | string or null | When group_by=api_key_id, this field provides the API key ID of the grouped usage result. |
| batch | boolean or null | When group_by=batch, this field tells whether the grouped usage result is batch or not. |
| input_audio_tokens | integer | The aggregated number of audio input tokens used, including cached tokens. |
| input_cached_tokens | integer | The aggregated number of text input tokens that has been cached from previous requests. For customers subscribe to scale tier, this includes scale tier tokens. |
| input_tokens | integer | The aggregated number of text input tokens used, including cached tokens. For customers subscribe to scale tier, this includes scale tier tokens. |
| model | string or null | When group_by=model, this field provides the model name of the grouped usage result. |
| num_model_requests | integer | The count of requests made to the model. |
| object | string |  |
| output_audio_tokens | integer | The aggregated number of audio output tokens used. |
| output_tokens | integer | The aggregated number of text output tokens used. For customers subscribe to scale tier, this includes scale tier tokens. |
| project_id | string or null | When group_by=project_id, this field provides the project ID of the grouped usage result. |
| user_id | string or null | When group_by=user_id, this field provides the user ID of the grouped usage result. |


# pricing

GPT-5
Input:
$1.250 / 1M tokens
Cached input:
$0.125 / 1M tokens
Output:
$10.000 / 1M tokens

GPT-5 mini
Input:
$0.250 / 1M tokens
Cached input:
$0.025 / 1M tokens
Output:
$2.000 / 1M tokens

GPT-5 nano
Input:
$0.050 / 1M tokens
Cached input:
$0.005 / 1M tokens
Output:
$0.400 / 1M tokens
