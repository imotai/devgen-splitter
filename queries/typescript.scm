(
  (comment)* @function.comment
  .
  (function_declaration
    name: (identifier) @function.name
  ) @function.definition
)

(
  (comment)* @arrow_function.comment
  .
  (lexical_declaration
    (variable_declarator
      name: (identifier) @arrow_function.name
      value: (arrow_function) @arrow_function.definition
    )
  )
)

(
  (comment)* @interface.comment
  .
  (interface_declaration
    name: (_) @interface.name
  ) @interface.definition
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
     
  )
)
