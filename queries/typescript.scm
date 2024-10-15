(
  (comment)* @function.comment
  .
  (function_declaration
    name: (identifier) @function.name
  ) @function.definition
)

(
  (comment)* @function.comment
  .
  (lexical_declaration
    (variable_declarator
      name: (identifier) @function.name
      value: (arrow_function) @function.body
    )
  ) @function.definition
)

(
  (comment)* @struct.comment
  .
  (interface_declaration
    name: (_) @struct.name
  ) @struct.definition
)

(
  (comment)* @enum.comment
  .
  (enum_declaration
    name: (_) @enum.name
  ) @enum.definition
)

(class_declaration
  name: (_) @method.class.name
  body: (class_body
  [
      (
        (comment)* @method.comment
        (method_definition
          name: (_) @method.name
        ) @method.definition
      )
      (
        (method_definition
          name: (_) @method.name
        ) @method.definition
      )
    ]* 
     
  )@class.definition
)
