(class_declaration
  name: (identifier) @method.class.name
  body: (class_body
    (
      (
        [
          (line_comment)
          (block_comment)
        ]* @method.comment
      )?
      .
      [
        (method_declaration
          name: (identifier) @method.name
        )
        (constructor_declaration
          name: (identifier) @method.name
        )
      ] @method.definition
    )*
  )
) 

(interface_declaration
  name: (identifier) @method.class.name
  body: (interface_body
    (
      (
        [
          (line_comment)
          (block_comment)
        ]* @method.comment
      )?
      .
      (method_declaration
        name: (identifier) @method.name
      ) @method.definition
    )*
  )
) 


(enum_declaration
  name: (identifier) @enum.name
) @enum.definition