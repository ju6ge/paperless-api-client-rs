def walk(f):
  # Recursively apply function `f` to all values in the JSON document.
  . as $in
  | if ($in | type) == "object" then
      (
        reduce ($in | keys_unsorted[]) as $k ({};
          .[$k] = ($in[$k] | walk(f))
        )
      )
      | f
    elif ($in | type) == "array" then
      (
        map(walk(f))
      )
      | f
    else
      f
    end;

# Ensure that any `all` property defined as an array has an `items.type`.
# If `items` is missing entirely, add it with `{ "type": "integer" }`.
# If `items` exists but doesn’t declare a `type` or a `$ref`, inject `"type": "integer"`.
walk(
  if type == "object"
     and has("all")
     and (.all | type == "object")
     and (.all.type == "array")
  then
    if (.all | has("items") | not) then
      .all.items = { "type": "integer" }
    elif (.all.items | type == "object"
          and (.all.items | has("type") | not)
          and (.all.items | has("$ref") | not)) then
      .all.items.type = "integer"
    else
      .
    end
  else
    .
  end
)
