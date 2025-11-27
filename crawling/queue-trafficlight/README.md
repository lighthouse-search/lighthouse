# Traffic Light
Hey Eddie. I built this in Rust because all my pipelines are in Rust and it'll be faster than me porting my pipelines to typescript. Sorry if Rust is annoying in the future :)

[Wrote this documentation at 6am so it might not have the best grammar sorry lol]

# Workflow
- On runner boot, a runner declares itself to Traffic Light via [POST /api/native-v1/runner/hello](#add-runner)
- Runner recieves a charger via webhook [POST https://[runner_ip]/internal/traffic-light/job/create](#receiving-a-job)
- As the runner progresses in the job, the runner uses [POST /api/native-v1/runner/update](#update-job) to coordinate job status with Traffic Light

# Notable values:
- ``Nonce``: When a runner boots, it generates a random string. This allows traffic-light to detect if a runner has rebooted, as it may experience amnesia to previous jobs. If the nonce is different, traffic-light (in the future, it presently doesn't) reassign all the runner's previous jobs.
- ``Capacity``: Rather than a hard job limit, I want traffic-light to limit jobs based on a runner's hardware capacity. I haven't figured out how this should be relatively calculated, so I've kept each runner to a hard limit of 3 jobs.

# Development
(Please make sure [Rust is installed](https://www.rust-lang.org/))

Run the following command to start traffic-light with the example configuration:

```
export trafficlight_config=$(cat ./example/config/basic.toml) && cargo run
```

You will need to define environment variables like ``password_env`` (stores the SQL password). For the example config, you'd run ``export example_user_mysql_password="my_sql_password"``. Traffic-light configurations never contain sensitive information, they instead reference environment variables.

# Runners
## Add runner
```POST /api/native-v1/runner/hello```
```
{
    "nonce": string
    "capacity": number
}
```

## Update runner capacity

```POST /api/native-v1/runner/update```
```
{
    "nonce": string
    "capacity": number
}
```

# Jobs

## Update job
```POST /api/native-v1/runner/update```

``status`` can be ``completed``, ``running``, ``stopped`` or ``error``
```
{
    "job_id": string,
    "status": string,
    "nonce": number
}
```

## Webhook a runner receives when being assigned a job
When traffic-light assigns a job to a runner, it will send:

```POST https://[runner_ip]/internal/traffic-light/job/create```
```
{
    "job_id": string,
    "cursor_to": number,
    "cursor_from": number
}
```
