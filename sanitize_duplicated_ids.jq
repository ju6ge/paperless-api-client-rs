def modify_param_list:
  # Rename the path parameter “id” to “doc_id” within an existing parameter array.
  if (. == null) then
    null
  else
    map(
      if (.in == "path" and .name == "id") then
        .name = "doc_id"
      else
        .
      end
    )
  end;

def modify_operation_params:
  # Modify the parameters list of an operation only if it exists.
  . as $op |
  if ($op.parameters != null) then
    .parameters = ($op.parameters | modify_param_list)
  else
    .
  end;

def modify_path_parameters:
  # Rename path-level 'id' to 'doc_id' for targeted path items and operations.
  (
    if (.parameters != null) then
      .parameters = (.parameters | modify_param_list)
    else
      .
    end
  )
  |
  # Update each HTTP method within the path item
  with_entries(
    if (.key | test("^(get|post|put|patch|delete|options|head|trace)$")) then
      .value = (.value | modify_operation_params)
    else
      .
    end
  );

# Main filter: rename only the document‑ID paths and their corresponding path parameters.
.paths |= (
  (to_entries
   | map(
       if (.key | test("/documents/\\{id\\}(/|$)")) then
         {
           key: (.key | gsub("\\{id\\}"; "{doc_id}")),
           value: (.value | modify_path_parameters)
         }
       else
         .
       end
     )
  ) | from_entries
)
