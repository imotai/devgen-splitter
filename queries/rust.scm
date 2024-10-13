(
      (line_comment)* @function.comment
      .
      (function_item
        name: (identifier) @function.name
      ) @function.definition
)

(
  (line_comment)* @struct.comment
  .
  (struct_item
    name: (type_identifier) @struct.name
  ) @struct.definition
)

(
  (line_comment)* @trait.comment
  .
  (trait_item
    name: (type_identifier) @trait.name
  ) @trait.definition
)

(impl_item
  trait: (type_identifier)? @impl.trait.name
  type: (type_identifier) @impl.class.name
  body: (declaration_list
    (
      (line_comment)* @method.comment
      .
      (function_item
        name: (identifier) @method.name
      ) @method.definition
    )
  )
)

(
  (line_comment)* @enum.comment
  .
  (enum_item
    name: (type_identifier) @enum.name
  ) @enum.definition
)
