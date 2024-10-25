; Class definitions with methods
(class_definition
  name: (identifier) @method.class.name
  body: (block
    (
      (
        [
          (comment)
        ]* @method.comment
      )?
      .
      (function_definition
        name: (identifier) @method.name
      ) @method.definition
    )*
  )
) @class.definition

; Standalone function definitions
(function_definition
  (
    [
      (comment)
    ]* @function.comment
  )?
  .
  name: (identifier) @function.name
) @function.definition