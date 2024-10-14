(
  (line_comment)* @interface.comment
  .
  (modifiers)? @interface.modifiers
  .
  (interface_declaration
    name: (identifier) @interface.name
    body: (interface_body
      (
        (line_comment)* @interface.method.comment
        .
        (annotation)* @interface.method.annotation
        .
        (method_declaration
          name: (identifier) @interface.method.name
        ) @interface.method.definition
      )*
    )
  ) @interface.definition
)

(
  (line_comment)* @enum.comment
  .
  (modifiers)? @enum.modifiers
  .
  (enum_declaration
    name: (identifier) @enum.name
  ) @enum.definition
)

; Methods within classes with class name
(class_declaration
  name: (identifier) @class.name
  body: (class_body
    (
      (line_comment)* @method.comment
      .
      (annotation)* @method.annotation
      .
      (method_declaration
        name: (identifier) @method.name
      ) @method.definition
    )*
  )
)

; Constructors
(
  (line_comment)* @constructor.comment
  .
  (constructor_declaration
    name: (identifier) @constructor.name
  ) @constructor.definition
)

