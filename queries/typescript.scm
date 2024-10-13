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
  (comment)* @class.comment
  .
  (class_declaration
    name: (_) @class.name
  ) @class.definition
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
  body: (class_body
    (
      (comment)+ @method.comment
      .
      (method_definition
        name: (_) @method.name
      ) @method.definition
    )
  )
)

(
  (comment)* @type.comment
  .
  (type_alias_declaration
    name: (_) @type.name
  ) @type.definition
)