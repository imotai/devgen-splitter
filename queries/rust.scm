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
  (attribute_item)? @struct.derive
  .
  (struct_item
    name: (type_identifier) @struct.name
  ) @struct.definition
)

(
  (line_comment)* @interface.comment
  .
  (attribute_item)? @interface.derive
  .
  (trait_item
    name: (type_identifier) @interface.name
  ) @interface.definition
)

(
  (line_comment)* @enum.comment
  .
  (attribute_item)? @enum.derive
  .
  (enum_item
    name: (type_identifier) @enum.name
  ) @enum.definition
)

(impl_item
  trait: (type_identifier)? @method.interface.name
  type: (type_identifier) @method.class.name
  body: (declaration_list
    (
      (line_comment)* @method.comment
      .
      (function_item
        name: (identifier) @method.name
      ) @method.definition
    )*
  )
)
