def blank_enum_targets:
  # Find enum schema names that are combined with BlankEnum.
  [
    .components.schemas[]?.properties?[]?
    | select(.oneOf? and (.oneOf | type == "array") and (.oneOf | any(."$ref"? == "#/components/schemas/BlankEnum")))
    | .oneOf[]
    | ."$ref"?
    | select(. != null and . != "#/components/schemas/BlankEnum" and . != "#/components/schemas/NullEnum")
    | split("/") | last
  ] | unique;

def add_blank_to_enums($targets):
  # Append "" to the enum values for these targets.
  .components.schemas |= with_entries(
    if (.key as $name | ($targets | index($name))) then
      .value |= (
        if (.enum? and (.enum | type) == "array") then
          .enum |= (
            if ((. | index(""))? // false) then . else . + [ ""] end
          )
        else
          .
        end
      )
    else . end
  );

def strip_blank_null_and_inline($spec):
  # Remove BlankEnum/NullEnum from `oneOf` and inline single-case enums.
  .components.schemas |= with_entries(
    .value |= (
      if (.properties? and (.properties | type == "object")) then
        .properties |= with_entries(
          .value |= (
            if (.oneOf? and (.oneOf | type == "array")) then
              # Drop BlankEnum and NullEnum references
              .oneOf = (.oneOf | map(select(."$ref" != "#/components/schemas/BlankEnum"
                                             and ."$ref" != "#/components/schemas/NullEnum"))) |
              # If exactly one case remains, inline type and enum
              if (.oneOf | length == 1) then
                (.oneOf[0]."$ref" as $ref |
                 if ($ref != null) then
                   ($ref | split("/") | last) as $enumName |
                   .type = $spec.components.schemas[$enumName].type |
                   .enum = $spec.components.schemas[$enumName].enum |
                   del(.oneOf)
                 else
                   .
                 end)
              else
                .
              end
            else
              .
            end
          )
        )
      else . end
    )
  );

# Main pipeline: add blank strings to the relevant enums, then update properties.
blank_enum_targets as $targets
| add_blank_to_enums($targets)
| . as $specWithEnums
| strip_blank_null_and_inline($specWithEnums)

