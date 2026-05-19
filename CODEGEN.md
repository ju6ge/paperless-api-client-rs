# Steps to run code genration

1. Go to `code-generation` branch 
2. Fetch latest paperless api specification:
```sh
curl 'https://{instance_url}/api/schema/' -H "Accept: application/json" > paperless-api.json 
```

3. Apply sanitization jq filters one after another:
   - sanitize_all.jq
   - sanitize_enums.jq
   - satitize_duplicated_ids.jq
   Use `jq -f <filter.jq> paperless-api.jq` write to a different json file for output the move the sanitized json to `paperless-api.json`
   
4. Commit this as new openapi.json
5. Rebase to move the commit forward in history: There where some manual patches applied to the api specification that do not have their own jq filter!
   - this step requires search for a sensible positon, probably directly after the last api spec update!
   
6. compile openapitor: `pushd openapitor && cargo build --release && popd`
7. create new base branch from `code-generation` branch
8. generate api client code with:
   `./openapitor/target/release/openapitor -i paperless-api.json -o . --base-url "https://your-paperles.url/api" --name paperless-api-client --target-version "6.0.1" --description "Paperless-ngx API client" --request-timeout-seconds 300`
9. Run clippy to auto format the generated code correctly!
   `cargo clippy --allow-dirty --fix --all --all-features -- -D clippy::uninlined_format_args`
   - This step might require some manual fixes to get it to compile …


