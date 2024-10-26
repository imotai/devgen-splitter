; Interface query
(
  (interface_declaration
    name: (identifier) @method.class.name
    body: (contract_body
      (
        (comment)* @method.comment
        .
        (function_definition
          name: (identifier) @method.name
        ) @method.definition
      )*
    )
  ) @class.definition
)

; Contract query
(
  (contract_declaration
    name: (identifier) @method.class.name
    body: (contract_body
      (_)*
      (
        (comment)* @method.comment
        .
        (function_definition
          name: (identifier) @method.name
        ) @method.definition
      )
      *
    )
  ) @class.definition
)

; Struct query
(
  (comment)* @struct.comment
  .
  (struct_declaration
    name: (identifier) @struct.name
  ) @struct.definition
)

; Enum query
(
  (comment)* @enum.comment
  .
  (enum_declaration
    name: (identifier) @enum.name
  ) @enum.definition
)

